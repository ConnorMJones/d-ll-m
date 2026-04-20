mod state;

use dllm_bindings::{
    DbConnection, dnd_5_e_item_table::Dnd5EItemTableAccess,
    dnd_5_e_monster_table::Dnd5EMonsterTableAccess, dnd_5_e_spell_table::Dnd5ESpellTableAccess,
    message_table::MessageTableAccess, send_message_reducer::send_message,
    set_name_reducer::set_name, user_table::UserTableAccess,
};
use spacetimedb_sdk::{DbContext, Identity, Table, TableWithPrimaryKey, Timestamp};
use state::{
    ConnectionRuntime, ItemRecord, MessageRecord, MonsterRecord, RuntimeState, SpellRecord,
    UserRecord,
};
use std::{
    fmt,
    sync::{Arc, Mutex},
};
use tracing::{error, info};

pub use state::{
    ClientConfig, ClientSnapshot, ConnectionStatus, ItemView, MessageView, MonsterView, SpellView,
    UserView,
};

pub const DEFAULT_URI: &str = "http://127.0.0.1:3033";
pub const DEFAULT_DATABASE_NAME: &str = "dllm";

#[derive(Debug)]
pub enum ClientError {
    AlreadyConnected,
    NotConnected,
    Connect(String),
    Request(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyConnected => write!(f, "client is already connected"),
            Self::NotConnected => write!(f, "client is not connected"),
            Self::Connect(message) => write!(f, "failed to connect: {message}"),
            Self::Request(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ClientError {}

#[derive(Clone)]
pub struct DllmClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    state: Arc<Mutex<RuntimeState>>,
    runtime: Mutex<Option<ConnectionRuntime>>,
}

impl Default for DllmClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DllmClient {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ClientInner {
                state: Arc::new(Mutex::new(RuntimeState::default())),
                runtime: Mutex::new(None),
            }),
        }
    }

    pub fn connect(&self, config: ClientConfig) -> Result<(), ClientError> {
        self.disconnect().ok();

        {
            let mut state = self.inner.state.lock().unwrap();
            state.reset_for_connect();
            state.connection_status = ConnectionStatus::Connecting;
        }

        let shared_state = self.inner.state.clone();
        let conn = DbConnection::builder()
            .with_uri(&config.uri)
            .with_database_name(&config.database_name)
            .on_connect(move |_ctx, identity, _token| {
                let mut state = shared_state.lock().unwrap();
                state.connection_status = ConnectionStatus::Connected;
                state.local_identity = Some(identity);
                state.last_error = None;
            })
            .on_connect_error({
                let shared_state = self.inner.state.clone();
                move |_ctx, err| {
                    let mut state = shared_state.lock().unwrap();
                    state.connection_status = ConnectionStatus::Disconnected;
                    state.last_error = Some(err.to_string());
                    error!(?err, "client connection failed");
                }
            })
            .on_disconnect({
                let shared_state = self.inner.state.clone();
                move |_ctx, err| {
                    let mut state = shared_state.lock().unwrap();
                    state.connection_status = ConnectionStatus::Disconnected;
                    if let Some(err) = err {
                        state.last_error = Some(err.to_string());
                        error!(?err, "client disconnected");
                    } else {
                        info!("client disconnected");
                    }
                }
            })
            .build()
            .map_err(|err| {
                self.record_error(err.to_string());
                ClientError::Connect(err.to_string())
            })?;

        register_callbacks(&conn, self.inner.state.clone());

        conn.subscription_builder()
            .on_applied({
                let shared_state = self.inner.state.clone();
                move |_ctx| {
                    let mut state = shared_state.lock().unwrap();
                    state.subscription_applied = true;
                    state.last_error = None;
                }
            })
            .on_error({
                let shared_state = self.inner.state.clone();
                move |_ctx, err| {
                    let mut state = shared_state.lock().unwrap();
                    state.subscription_applied = false;
                    state.last_error = Some(err.to_string());
                    error!(?err, "subscription failed");
                }
            })
            .subscribe([
                "SELECT * FROM user",
                "SELECT * FROM message",
                "SELECT * FROM dnd_5_e_spell",
                "SELECT * FROM dnd_5_e_monster",
                "SELECT * FROM dnd_5_e_item",
            ]);

        let handle = conn.run_threaded();
        let mut runtime = self.inner.runtime.lock().unwrap();
        if runtime.is_some() {
            self.record_error("client is already connected".to_string());
            return Err(ClientError::AlreadyConnected);
        }
        *runtime = Some(ConnectionRuntime::new(conn, handle));

        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), ClientError> {
        let runtime = self.inner.runtime.lock().unwrap().take();
        if let Some(runtime) = runtime {
            runtime.conn.disconnect().ok();
            runtime.join();
        }

        let mut state = self.inner.state.lock().unwrap();
        state.connection_status = ConnectionStatus::Disconnected;
        state.subscription_applied = false;
        Ok(())
    }

    pub fn set_name(&self, name: impl Into<String>) -> Result<(), ClientError> {
        let name = name.into();
        if name.trim().is_empty() {
            self.record_error("name cannot be empty".to_string());
            return Err(ClientError::Request("name cannot be empty".to_string()));
        }

        let shared_state = self.inner.state.clone();
        let result = {
            let runtime = self.inner.runtime.lock().unwrap();
            let runtime = runtime.as_ref().ok_or_else(|| {
                self.record_error("client is not connected".to_string());
                ClientError::NotConnected
            })?;
            runtime
                .conn
                .reducers
                .set_name_then(name, move |_ctx, result| {
                    if let Err(message) = reducer_result_to_message(result) {
                        let mut state = shared_state.lock().unwrap();
                        state.last_error = Some(message);
                    }
                })
        };

        result.map_err(|err| {
            self.record_error(err.to_string());
            ClientError::Request(err.to_string())
        })
    }

    pub fn send_message(&self, text: impl Into<String>) -> Result<(), ClientError> {
        let text = text.into();
        if text.trim().is_empty() {
            self.record_error("message cannot be empty".to_string());
            return Err(ClientError::Request("message cannot be empty".to_string()));
        }

        let shared_state = self.inner.state.clone();
        let result = {
            let runtime = self.inner.runtime.lock().unwrap();
            let runtime = runtime.as_ref().ok_or_else(|| {
                self.record_error("client is not connected".to_string());
                ClientError::NotConnected
            })?;
            runtime
                .conn
                .reducers
                .send_message_then(text, move |_ctx, result| {
                    if let Err(message) = reducer_result_to_message(result) {
                        let mut state = shared_state.lock().unwrap();
                        state.last_error = Some(message);
                    }
                })
        };

        result.map_err(|err| {
            self.record_error(err.to_string());
            ClientError::Request(err.to_string())
        })
    }

    pub fn snapshot(&self) -> ClientSnapshot {
        let state = self.inner.state.lock().unwrap();
        state.snapshot()
    }

    fn record_error(&self, message: String) {
        let mut state = self.inner.state.lock().unwrap();
        state.last_error = Some(message);
    }
}

fn register_callbacks(conn: &DbConnection, shared_state: Arc<Mutex<RuntimeState>>) {
    conn.db.user().on_insert({
        let shared_state = shared_state.clone();
        move |_ctx, user| {
            shared_state
                .lock()
                .unwrap()
                .users
                .insert(identity_key(&user.identity), UserRecord::from_row(user));
        }
    });
    conn.db.user().on_update({
        let shared_state = shared_state.clone();
        move |_ctx, _old, new| {
            shared_state
                .lock()
                .unwrap()
                .users
                .insert(identity_key(&new.identity), UserRecord::from_row(new));
        }
    });
    conn.db.user().on_delete({
        let shared_state = shared_state.clone();
        move |_ctx, user| {
            shared_state
                .lock()
                .unwrap()
                .users
                .remove(&identity_key(&user.identity));
        }
    });

    conn.db.message().on_insert({
        let shared_state = shared_state.clone();
        move |_ctx, message| {
            let mut state = shared_state.lock().unwrap();
            let record = MessageRecord::from_row(message);
            state.upsert_message(record);
        }
    });
    conn.db.message().on_update({
        let shared_state = shared_state.clone();
        move |_ctx, _old, new| {
            let mut state = shared_state.lock().unwrap();
            let record = MessageRecord::from_row(new);
            state.upsert_message(record);
        }
    });
    conn.db.message().on_delete({
        let shared_state = shared_state.clone();
        move |_ctx, message| {
            shared_state
                .lock()
                .unwrap()
                .messages
                .retain(|m| m.id != message.id);
        }
    });

    conn.db.dnd_5_e_spell().on_insert({
        let shared_state = shared_state.clone();
        move |_ctx, spell| {
            let record = SpellRecord::from_row(spell);
            shared_state
                .lock()
                .unwrap()
                .spells
                .insert(record.id, record);
        }
    });
    conn.db.dnd_5_e_spell().on_update({
        let shared_state = shared_state.clone();
        move |_ctx, _old, new| {
            let record = SpellRecord::from_row(new);
            shared_state
                .lock()
                .unwrap()
                .spells
                .insert(record.id, record);
        }
    });
    conn.db.dnd_5_e_spell().on_delete({
        let shared_state = shared_state.clone();
        move |_ctx, spell| {
            shared_state.lock().unwrap().spells.remove(&spell.id);
        }
    });

    conn.db.dnd_5_e_monster().on_insert({
        let shared_state = shared_state.clone();
        move |_ctx, monster| {
            let record = MonsterRecord::from_row(monster);
            shared_state
                .lock()
                .unwrap()
                .monsters
                .insert(record.id, record);
        }
    });
    conn.db.dnd_5_e_monster().on_update({
        let shared_state = shared_state.clone();
        move |_ctx, _old, new| {
            let record = MonsterRecord::from_row(new);
            shared_state
                .lock()
                .unwrap()
                .monsters
                .insert(record.id, record);
        }
    });
    conn.db.dnd_5_e_monster().on_delete({
        let shared_state = shared_state.clone();
        move |_ctx, monster| {
            shared_state.lock().unwrap().monsters.remove(&monster.id);
        }
    });

    conn.db.dnd_5_e_item().on_insert({
        let shared_state = shared_state.clone();
        move |_ctx, item| {
            let record = ItemRecord::from_row(item);
            shared_state.lock().unwrap().items.insert(record.id, record);
        }
    });
    conn.db.dnd_5_e_item().on_update({
        let shared_state = shared_state.clone();
        move |_ctx, _old, new| {
            let record = ItemRecord::from_row(new);
            shared_state.lock().unwrap().items.insert(record.id, record);
        }
    });
    conn.db.dnd_5_e_item().on_delete({
        let shared_state: Arc<Mutex<RuntimeState>> = shared_state.clone();
        move |_ctx, item| {
            shared_state.lock().unwrap().items.remove(&item.id);
        }
    });
}

fn reducer_result_to_message(
    result: Result<Result<(), String>, impl ToString>,
) -> Result<(), String> {
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(message)) => Err(message),
        Err(err) => Err(err.to_string()),
    }
}

fn identity_key(identity: &Identity) -> String {
    identity.to_string()
}

fn timestamp_text(timestamp: &Timestamp) -> String {
    format!("{timestamp:?}")
}

impl RuntimeState {
    fn snapshot(&self) -> ClientSnapshot {
        let mut users = self.users.values().cloned().collect::<Vec<_>>();
        users.sort_by(|a, b| {
            a.display_name()
                .cmp(&b.display_name())
                .then_with(|| identity_key(&a.identity).cmp(&identity_key(&b.identity)))
        });

        let mut messages = self.messages.clone();
        messages.sort_by_key(|m| m.id);

        let mut spells = self.spells.values().cloned().collect::<Vec<_>>();
        spells.sort_by(|a, b| a.name.cmp(&b.name));

        let mut monsters = self.monsters.values().cloned().collect::<Vec<_>>();
        monsters.sort_by(|a, b| a.name.cmp(&b.name));

        let mut items = self.items.values().cloned().collect::<Vec<_>>();
        items.sort_by(|a, b| a.name.cmp(&b.name));

        ClientSnapshot {
            connection_status: self.connection_status,
            subscription_applied: self.subscription_applied,
            local_identity: self.local_identity.as_ref().map(identity_key),
            users: users
                .into_iter()
                .map(|u| UserView {
                    identity: identity_key(&u.identity),
                    name: u.name,
                    online: u.online,
                })
                .collect(),
            messages: messages
                .into_iter()
                .map(|m| MessageView {
                    id: m.id,
                    sender_identity: identity_key(&m.sender),
                    sender_name: self
                        .users
                        .get(&identity_key(&m.sender))
                        .and_then(|u| u.name.clone())
                        .unwrap_or_else(|| "anonymous".to_string()),
                    text: m.text,
                    sent: timestamp_text(&m.sent),
                })
                .collect(),
            spells: spells
                .into_iter()
                .map(|s| SpellView {
                    id: s.id,
                    name: s.name,
                    level: s.level,
                    school: s.school,
                    ritual: s.ritual,
                    concentration: s.concentration,
                    description: s.description,
                })
                .collect(),
            monsters: monsters
                .into_iter()
                .map(|m| MonsterView {
                    id: m.id,
                    name: m.name,
                    cr: m.cr,
                    size: m.size,
                    creature_type: m.creature_type,
                    ac: m.ac,
                    hp_average: m.hp_average,
                    description: m.description,
                })
                .collect(),
            items: items
                .into_iter()
                .map(|i| ItemView {
                    id: i.id,
                    name: i.name,
                    item_type: i.item_type,
                    rarity: i.rarity,
                    description: i.description,
                })
                .collect(),
            last_error: self.last_error.clone(),
        }
    }

    fn upsert_message(&mut self, message: MessageRecord) {
        if let Some(existing) = self.messages.iter_mut().find(|m| m.id == message.id) {
            *existing = message;
        } else {
            self.messages.push(message);
        }
    }
}

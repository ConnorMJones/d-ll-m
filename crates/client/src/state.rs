use dllm_bindings::{message_type::Message, user_type::User};
use spacetimedb_sdk::{Identity, Timestamp};
use std::{collections::BTreeMap, thread::JoinHandle};

use crate::{DEFAULT_DATABASE_NAME, DEFAULT_URI};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub uri: String,
    pub database_name: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            uri: DEFAULT_URI.to_string(),
            database_name: DEFAULT_DATABASE_NAME.to_string(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientSnapshot {
    pub connection_status: ConnectionStatus,
    pub subscription_applied: bool,
    pub local_identity: Option<String>,
    pub users: Vec<UserView>,
    pub messages: Vec<MessageView>,
    pub last_error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserView {
    pub identity: String,
    pub name: Option<String>,
    pub online: bool,
}

impl UserView {
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("anonymous")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageView {
    pub id: u64,
    pub sender_identity: String,
    pub sender_name: String,
    pub text: String,
    pub sent: String,
}

pub struct ConnectionRuntime {
    pub conn: dllm_bindings::DbConnection,
    thread: Option<JoinHandle<()>>,
}

impl ConnectionRuntime {
    pub fn new(conn: dllm_bindings::DbConnection, thread: JoinHandle<()>) -> Self {
        Self {
            conn,
            thread: Some(thread),
        }
    }

    pub fn join(mut self) {
        if let Some(thread) = self.thread.take() {
            if let Err(err) = thread.join() {
                tracing::warn!(?err, "client connection thread ended unexpectedly");
            }
        }
    }
}

#[derive(Default)]
pub struct RuntimeState {
    pub connection_status: ConnectionStatus,
    pub subscription_applied: bool,
    pub local_identity: Option<Identity>,
    pub users: BTreeMap<String, UserRecord>,
    pub messages: Vec<MessageRecord>,
    pub last_error: Option<String>,
}

impl RuntimeState {
    pub fn reset_for_connect(&mut self) {
        self.connection_status = ConnectionStatus::Disconnected;
        self.subscription_applied = false;
        self.local_identity = None;
        self.users.clear();
        self.messages.clear();
        self.last_error = None;
    }
}

#[derive(Clone)]
pub struct UserRecord {
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,
}

impl UserRecord {
    pub fn from_row(row: &User) -> Self {
        Self {
            identity: row.identity,
            name: row.name.clone(),
            online: row.online,
        }
    }

    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "anonymous".to_string())
    }
}

#[derive(Clone)]
pub struct MessageRecord {
    pub id: u64,
    pub sender: Identity,
    pub text: String,
    pub sent: Timestamp,
}

impl MessageRecord {
    pub fn from_row(row: &Message) -> Self {
        Self {
            id: row.id,
            sender: row.sender,
            text: row.text.clone(),
            sent: row.sent,
        }
    }
}

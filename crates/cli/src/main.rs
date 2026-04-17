use dllm_client::{
    DbConnection, message_table::MessageTableAccess, send_message_reducer::send_message,
    set_name_reducer::set_name, user_table::UserTableAccess,
};
use spacetimedb_sdk::{DbContext, Table, TableWithPrimaryKey};
use std::io::{self, BufRead, Write};

const SPACETIMEDB_URI: &str = "http://127.0.0.1:3033";
const DB_NAME: &str = "dllm";

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("Connecting to SpacetimeDB at {}...", SPACETIMEDB_URI);

    let conn = DbConnection::builder()
        .with_uri(SPACETIMEDB_URI)
        .with_database_name(DB_NAME)
        .on_connect(|_ctx, _identity, _token| {
            println!("Connected!");
        })
        .on_connect_error(|_ctx, err| {
            eprintln!("Connection error: {:?}", err);
        })
        .on_disconnect(|_ctx, err| {
            if let Some(err) = err {
                eprintln!("Disconnected with error: {:?}", err);
            } else {
                println!("Disconnected.");
            }
        })
        .build()
        .expect("Failed to connect");

    conn.subscription_builder()
        .on_applied(|_ctx| {
            println!("Subscribed to tables.");
        })
        .subscribe(["SELECT * FROM user", "SELECT * FROM message"]);

    conn.db.user().on_insert(|_ctx, user| {
        let name = user.name.as_deref().unwrap_or("anonymous");
        if user.online {
            println!("[{}] came online", name);
        }
    });

    conn.db.user().on_update(|_ctx, old, new| {
        if old.online != new.online {
            let name = new.name.as_deref().unwrap_or("anonymous");
            if new.online {
                println!("[{}] came online", name);
            } else {
                println!("[{}] went offline", name);
            }
        }
        if old.name != new.name {
            let old_name = old.name.as_deref().unwrap_or("anonymous");
            let new_name = new.name.as_deref().unwrap_or("anonymous");
            println!("[{}] changed name to [{}]", old_name, new_name);
        }
    });

    conn.db.message().on_insert(|ctx, msg| {
        let sender_name = ctx
            .db
            .user()
            .identity()
            .find(&msg.sender)
            .map(|u| u.name.clone().unwrap_or_else(|| "anonymous".to_string()))
            .unwrap_or_else(|| "unknown".to_string());
        println!("<{}> {}", sender_name, msg.text);
    });

    let handle = conn.run_threaded();

    println!("Commands: /name <name>, /quit, or just type to send a message");
    println!();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "/quit" {
            conn.disconnect().ok();
            break;
        } else if let Some(name) = line.strip_prefix("/name ") {
            conn.reducers.set_name(name.to_string()).ok();
        } else {
            conn.reducers.send_message(line.to_string()).ok();
        }

        io::stdout().flush().ok();
    }

    handle.join().ok();
}

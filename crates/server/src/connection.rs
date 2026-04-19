use dllm_bindings::DbConnection;
use tracing::{error, info};

pub fn connect(uri: &str, database_name: &str) -> DbConnection {
    info!(uri, database_name, "connecting to SpacetimeDB");

    DbConnection::builder()
        .with_uri(uri)
        .with_database_name(database_name)
        .on_connect(|_ctx, _identity, _token| {
            info!("connected");
        })
        .on_connect_error(|_ctx, err| {
            error!(?err, "connection failed");
            std::process::exit(1);
        })
        .build()
        .expect("Failed to connect")
}

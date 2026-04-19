mod query;
use data_import::dnd5e as import_dnd5e;
use dllm_server::connect;
use spacetimedb_sdk::DbContext;
use std::path::Path;
use tracing::info;

const SPACETIMEDB_URI: &str = "http://127.0.0.1:3033";
const DB_NAME: &str = "dllm";

pub fn seed(data_dir: &Path) {
    let conn = connect(SPACETIMEDB_URI, DB_NAME);
    let _handle = conn.run_threaded();
    std::thread::sleep(std::time::Duration::from_secs(1));

    let report = import_dnd5e::import(&conn, data_dir);

    info!("waiting for inserts to complete");
    std::thread::sleep(std::time::Duration::from_secs(5));
    conn.disconnect().ok();

    import_dnd5e::log_report(&report);
    info!("done");
}

pub fn query_spells(name_filter: Option<String>, level_filter: Option<u8>) {
    let conn = connect(SPACETIMEDB_URI, DB_NAME);
    query::query_spells(&conn, name_filter, level_filter);
    conn.disconnect().ok();
}

pub fn query_monsters(name_filter: Option<String>, cr_filter: Option<String>) {
    let conn = connect(SPACETIMEDB_URI, DB_NAME);
    query::query_monsters(&conn, name_filter, cr_filter);
    conn.disconnect().ok();
}

pub fn query_items(name_filter: Option<String>, rarity_filter: Option<String>) {
    let conn = connect(SPACETIMEDB_URI, DB_NAME);
    query::query_items(&conn, name_filter, rarity_filter);
    conn.disconnect().ok();
}

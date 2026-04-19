use super::support::read_json_file;
use crate::dnd5e::normalize::normalize_item;
use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::ItemFile;
use crate::dnd5e::write;
use dllm_bindings::DbConnection;
use std::path::Path;
use tracing::info;

pub fn seed(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    info!(?data_dir, "reading items");

    let mut report = SectionReport::new("items");

    for filename in ["items.json", "items-base.json"] {
        let path = data_dir.join(filename);
        if !path.exists() {
            info!(filename, "skipping (not found)");
            continue;
        }

        let Some(item_file) = read_json_file::<ItemFile>(&path, &mut report) else {
            continue;
        };

        for raw_item in item_file.item.into_iter().chain(item_file.baseitem) {
            report.note_seen();
            let item = match normalize_item(raw_item, &mut report) {
                Some(item) => item,
                None => continue,
            };
            let item_name = format!("{} [{}]", item.name, item.source);

            match write::item(conn, item) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded items");
    report
}

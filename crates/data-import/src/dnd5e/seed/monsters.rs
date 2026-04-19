use super::support::{json_files_with_prefix, read_json_file};
use crate::dnd5e::normalize::normalize_monster;
use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::MonsterFile;
use crate::dnd5e::write;
use dllm_bindings::DbConnection;
use std::path::Path;
use tracing::info;

pub fn seed(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let bestiary_dir = data_dir.join("bestiary");
    info!(?bestiary_dir, "reading monsters");

    let mut report = SectionReport::new("monsters");

    for (_, path) in json_files_with_prefix(&bestiary_dir, "bestiary-") {
        let Some(monster_file) = read_json_file::<MonsterFile>(&path, &mut report) else {
            continue;
        };

        for raw_monster in monster_file.monster {
            report.note_seen();
            let monster = match normalize_monster(raw_monster, &mut report) {
                Some(monster) => monster,
                None => continue,
            };
            let item_name = format!("{} [{}]", monster.name, monster.source);

            match write::monster(conn, monster) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded monsters");
    report
}

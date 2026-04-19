use super::support::{json_files_with_prefix, read_json_file};
use crate::dnd5e::normalize::normalize_spell;
use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::SpellFile;
use crate::dnd5e::write;
use dllm_bindings::DbConnection;
use std::path::Path;
use tracing::info;

pub fn seed(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let spells_dir = data_dir.join("spells");
    info!(?spells_dir, "reading spells");

    let mut report = SectionReport::new("spells");

    for (_, path) in json_files_with_prefix(&spells_dir, "spell-") {
        let Some(spell_file) = read_json_file::<SpellFile>(&path, &mut report) else {
            continue;
        };

        for raw_spell in spell_file.spell {
            report.note_seen();
            let spell = normalize_spell(raw_spell, &mut report);
            let item_name = format!("{} [{}]", spell.name, spell.source);

            match write::spell(conn, spell) {
                Ok(()) => report.imported(),
                Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
            }
        }
    }

    info!(count = report.imported, "seeded spells");
    report
}

mod classes;
use super::report::ImportReport;
mod items;
mod monsters;
mod other;
mod references;
mod spells;
pub(crate) mod support;

use dllm_bindings::DbConnection;
use std::path::Path;

pub fn seed_all(conn: &DbConnection, data_dir: &Path) -> ImportReport {
    ImportReport {
        sections: vec![
            spells::seed(conn, data_dir),
            monsters::seed(conn, data_dir),
            items::seed(conn, data_dir),
            classes::seed_classes(conn, data_dir),
            classes::seed_subclasses(conn, data_dir),
            classes::seed_class_features(conn, data_dir),
            classes::seed_subclass_features(conn, data_dir),
            other::seed_actions(conn, data_dir),
            other::seed_feats(conn, data_dir),
            other::seed_conditions(conn, data_dir),
            other::seed_backgrounds(conn, data_dir),
            other::seed_races(conn, data_dir),
            other::seed_optional_features(conn, data_dir),
            other::seed_languages(conn, data_dir),
            other::seed_senses(conn, data_dir),
            other::seed_skills(conn, data_dir),
            references::seed_objects(conn, data_dir),
            references::seed_vehicles(conn, data_dir),
            references::seed_deities(conn, data_dir),
            references::seed_rewards(conn, data_dir),
            references::seed_trap_hazards(conn, data_dir),
            references::seed_char_creation_options(conn, data_dir),
            references::seed_psionics(conn, data_dir),
            references::seed_recipes(conn, data_dir),
            references::seed_cults_boons(conn, data_dir),
            references::seed_decks(conn, data_dir),
            references::seed_variant_rules(conn, data_dir),
        ],
    }
}

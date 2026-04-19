use super::report::ImportReport;
mod items;
mod monsters;
mod other;
mod spells;
pub(crate) mod support;

use dllm_client::DbConnection;
use std::path::Path;

pub fn seed_all(conn: &DbConnection, data_dir: &Path) -> ImportReport {
    ImportReport {
        sections: vec![
            spells::seed(conn, data_dir),
            monsters::seed(conn, data_dir),
            items::seed(conn, data_dir),
            other::seed_actions(conn, data_dir),
            other::seed_feats(conn, data_dir),
            other::seed_conditions(conn, data_dir),
            other::seed_backgrounds(conn, data_dir),
            other::seed_races(conn, data_dir),
            other::seed_optional_features(conn, data_dir),
            other::seed_languages(conn, data_dir),
            other::seed_senses(conn, data_dir),
            other::seed_skills(conn, data_dir),
        ],
    }
}

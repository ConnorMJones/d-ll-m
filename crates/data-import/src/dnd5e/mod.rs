mod entry;
mod normalize;
mod report;
mod reporting;
mod seed;
mod types;
mod write;

use dllm_bindings::DbConnection;
use std::path::Path;

pub use report::{ImportIssue, ImportReport, SectionReport};
pub use reporting::log_report;

pub fn import(conn: &DbConnection, data_dir: &Path) -> ImportReport {
    seed::seed_all(conn, data_dir)
}

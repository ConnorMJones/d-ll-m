use super::report::{ImportReport, SectionReport};
use tracing::{info, warn};

pub fn log_report(report: &ImportReport) {
    info!(
        seen = report.total_seen(),
        imported = report.total_imported(),
        skipped = report.total_skipped(),
        failed = report.total_failed(),
        warnings = report.total_warnings(),
        "import summary"
    );

    for section in &report.sections {
        log_section(section);
    }
}

fn log_section(section: &SectionReport) {
    info!(
        section = section.name,
        seen = section.seen,
        imported = section.imported,
        skipped = section.skipped,
        failed = section.failed,
        warnings = section.warnings.len(),
        "import section"
    );

    for issue in &section.warnings {
        match &issue.item {
            Some(item) => {
                warn!(section = section.name, item, detail = %issue.detail, "import issue")
            }
            None => warn!(section = section.name, detail = %issue.detail, "import issue"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportIssue {
    pub item: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct SectionReport {
    pub name: &'static str,
    pub seen: u64,
    pub imported: u64,
    pub skipped: u64,
    pub failed: u64,
    pub warnings: Vec<ImportIssue>,
}

impl SectionReport {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            seen: 0,
            imported: 0,
            skipped: 0,
            failed: 0,
            warnings: Vec::new(),
        }
    }

    pub fn note_seen(&mut self) {
        self.seen += 1;
    }

    pub fn imported(&mut self) {
        self.imported += 1;
    }

    pub fn skipped(&mut self, item: impl Into<String>, detail: impl Into<String>) {
        self.skipped += 1;
        self.warn(Some(item.into()), detail);
    }

    pub fn failed(&mut self, item: impl Into<String>, detail: impl Into<String>) {
        self.failed += 1;
        self.warn(Some(item.into()), detail);
    }

    pub fn warn(&mut self, item: Option<String>, detail: impl Into<String>) {
        self.warnings.push(ImportIssue {
            item,
            detail: detail.into(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct ImportReport {
    pub sections: Vec<SectionReport>,
}

impl ImportReport {
    pub fn total_seen(&self) -> u64 {
        self.sections.iter().map(|s| s.seen).sum()
    }

    pub fn total_imported(&self) -> u64 {
        self.sections.iter().map(|s| s.imported).sum()
    }

    pub fn total_skipped(&self) -> u64 {
        self.sections.iter().map(|s| s.skipped).sum()
    }

    pub fn total_failed(&self) -> u64 {
        self.sections.iter().map(|s| s.failed).sum()
    }

    pub fn total_warnings(&self) -> usize {
        self.sections.iter().map(|s| s.warnings.len()).sum()
    }
}

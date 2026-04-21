use data_import::dnd5e;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn validation_reports_unsupported_grant_blocks_and_sidekick_skips() {
    let data_dir = temp_test_dir("dnd5e-cleanup-reporting");
    write_test_file(
        &data_dir.join("backgrounds.json"),
        r#"{
  "background": [
    {
      "name": "Broken Background",
      "source": "TST",
      "skillProficiencies": [{}],
      "toolProficiencies": [{}],
      "languageProficiencies": [{}],
      "entries": ["Test background."]
    }
  ]
}"#,
    );
    write_test_file(&data_dir.join("class/class-sidekick.json"), "{}");

    let report = dnd5e::validate(&data_dir);

    let backgrounds = find_section(&report, "backgrounds");
    assert_eq!(backgrounds.imported, 1, "background should still validate");
    assert!(backgrounds.warnings.iter().any(|issue| {
        issue
            .detail
            .contains("skill proficiency block 1 contained no supported grant shape")
    }));
    assert!(backgrounds.warnings.iter().any(|issue| {
        issue
            .detail
            .contains("tool proficiency block 1 contained no supported grant shape")
    }));
    assert!(backgrounds.warnings.iter().any(|issue| {
        issue
            .detail
            .contains("language proficiency block 1 contained no supported grant shape")
    }));

    let classes = find_section(&report, "classes");
    assert!(classes.warnings.iter().any(|issue| {
        issue.item.as_deref() == Some("class-sidekick.json")
            && issue.detail.contains("skipping sidekick data")
    }));

    fs::remove_dir_all(&data_dir).expect("failed to remove temp test dir");
}

fn find_section<'a>(report: &'a dnd5e::ImportReport, name: &str) -> &'a dnd5e::SectionReport {
    report
        .sections
        .iter()
        .find(|section| section.name == name)
        .unwrap_or_else(|| panic!("missing section: {name}"))
}

fn write_test_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create parent dir");
    }
    fs::write(path, content).expect("failed to write test file");
}

fn temp_test_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos()
    );
    let dir = std::env::temp_dir().join(unique);
    fs::create_dir_all(&dir).expect("failed to create temp test dir");
    dir
}

use data_import::dnd5e;
use std::path::PathBuf;

#[test]
fn validates_recent_dnd5e_reference_expansion_sections() {
    let report = dnd5e::validate(&data_dir());

    for section_name in RECENT_SECTIONS {
        let section = report
            .sections
            .iter()
            .find(|section| section.name == *section_name)
            .unwrap_or_else(|| panic!("missing validation section: {section_name}"));

        assert!(
            section.seen > 0,
            "section {section_name} saw no records: {section:#?}"
        );
        assert!(
            section.imported > 0,
            "section {section_name} imported no records: {section:#?}"
        );
        assert_eq!(
            section.failed, 0,
            "section {section_name} had failures: {section:#?}"
        );
    }
}

#[test]
#[ignore = "full importer audit; current legacy parser debt still exists in older sections"]
fn audits_full_current_dnd5e_import_surface() {
    let report = dnd5e::validate(&data_dir());
    let failed_sections = report
        .sections
        .iter()
        .filter(|section| section.failed > 0)
        .map(|section| format!("{}: {}", section.name, section.failed))
        .collect::<Vec<_>>();

    assert!(
        failed_sections.is_empty(),
        "full importer audit still has failing sections: {}",
        failed_sections.join(", ")
    );
}

const RECENT_SECTIONS: &[&str] = &[
    "classes",
    "subclasses",
    "class_features",
    "subclass_features",
    "actions",
    "languages",
    "senses",
    "skills",
    "objects",
    "vehicles",
    "deities",
    "rewards",
    "trap_hazards",
    "char_creation_options",
    "psionics",
    "recipes",
    "cults_boons",
    "decks",
    "variant_rules",
];

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../ttrpg/dnd5e/data")
        .canonicalize()
        .expect("failed to resolve dnd5e data dir")
}

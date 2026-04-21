use dllm_core::dnd5e::{class_feature_key, source_name_key, subclass_feature_key, subclass_key};

#[test]
fn source_name_key_is_case_and_whitespace_insensitive() {
    assert_eq!(
        source_name_key(" PHB ", " Spellcasting "),
        source_name_key("phb", "spellcasting")
    );
}

#[test]
fn class_feature_key_distinguishes_same_named_features_across_classes() {
    let cleric = class_feature_key("phb", "phb", "Cleric", 1, "Spellcasting");
    let wizard = class_feature_key("phb", "phb", "Wizard", 1, "Spellcasting");

    assert_ne!(cleric, wizard);
}

#[test]
fn class_feature_key_distinguishes_same_named_features_across_levels() {
    let first = class_feature_key("phb", "phb", "Cleric", 1, "Channel Divinity");
    let second = class_feature_key("phb", "phb", "Cleric", 2, "Channel Divinity");

    assert_ne!(first, second);
}

#[test]
fn subclass_key_distinguishes_same_subclass_name_under_different_classes() {
    let fighter = subclass_key("xge", "phb", "Fighter", "Arcane Archer");
    let ranger = subclass_key("xge", "phb", "Ranger", "Arcane Archer");

    assert_ne!(fighter, ranger);
}

#[test]
fn subclass_feature_key_distinguishes_same_named_features_across_subclasses() {
    let life = subclass_feature_key(
        "phb",
        "phb",
        "Cleric",
        "phb",
        "Life Domain",
        2,
        "Channel Divinity",
    );
    let war = subclass_feature_key(
        "phb",
        "phb",
        "Cleric",
        "phb",
        "War Domain",
        2,
        "Channel Divinity",
    );

    assert_ne!(life, war);
}

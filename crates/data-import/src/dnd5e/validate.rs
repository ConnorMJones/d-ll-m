use super::normalize::{
    normalize_action, normalize_background, normalize_char_creation_option, normalize_class,
    normalize_class_feature, normalize_condition, normalize_cult_boon, normalize_deck,
    normalize_deity, normalize_feat, normalize_language, normalize_monster, normalize_object,
    normalize_optional_feature, normalize_psionic, normalize_race, normalize_recipe,
    normalize_reward, normalize_sense, normalize_skill, normalize_spell, normalize_subclass,
    normalize_subclass_feature, normalize_trap_hazard, normalize_variant_rule, normalize_vehicle,
};
use super::report::{ImportReport, SectionReport};
use super::seed::support::{json_files_with_prefix, read_json_file, warn_skipped_class_sidekick};
use super::types::{
    ActionFile, BackgroundFile, CharCreationOptionFile, ClassFile, ConditionFile, CultsBoonsFile,
    DeckFile, DeityFile, FeatFile, LanguageFile, MonsterFile, ObjectFile, OptionalFeatureFile,
    PsionicFile, RaceFile, RecipeFile, RewardFile, SenseFile, SkillFile, SpellFile, TrapHazardFile,
    VariantRuleFile, VehicleFile,
};
use std::path::Path;
use tracing::info;

pub fn validate_all(data_dir: &Path) -> ImportReport {
    ImportReport {
        sections: vec![
            validate_spells(data_dir),
            validate_monsters(data_dir),
            validate_items(data_dir),
            validate_classes(data_dir),
            validate_subclasses(data_dir),
            validate_class_features(data_dir),
            validate_subclass_features(data_dir),
            validate_actions(data_dir),
            validate_feats(data_dir),
            validate_conditions(data_dir),
            validate_backgrounds(data_dir),
            validate_races(data_dir),
            validate_optional_features(data_dir),
            validate_languages(data_dir),
            validate_senses(data_dir),
            validate_skills(data_dir),
            validate_objects(data_dir),
            validate_vehicles(data_dir),
            validate_deities(data_dir),
            validate_rewards(data_dir),
            validate_trap_hazards(data_dir),
            validate_char_creation_options(data_dir),
            validate_psionics(data_dir),
            validate_recipes(data_dir),
            validate_cults_boons(data_dir),
            validate_decks(data_dir),
            validate_variant_rules(data_dir),
        ],
    }
}

fn validate_infallible<T>(
    report: &mut SectionReport,
    normalize: impl FnOnce(&mut SectionReport) -> T,
) {
    let _ = normalize(report);
    report.imported();
}

fn validate_optional<T>(
    report: &mut SectionReport,
    normalize: impl FnOnce(&mut SectionReport) -> Option<T>,
) {
    if normalize(report).is_some() {
        report.imported();
    }
}

fn validate_spells(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("spells");
    let mut report = SectionReport::new("spells");

    if !path.exists() {
        report.warn(None, "spells directory not found");
        return report;
    }

    for (_filename, path) in json_files_with_prefix(&path, "spells-") {
        let Some(file) = read_json_file::<SpellFile>(&path, &mut report) else {
            continue;
        };

        for raw in file.spell {
            report.note_seen();
            validate_infallible(&mut report, |report| normalize_spell(raw, report));
        }
    }

    info!(count = report.imported, "validated spells");
    report
}

fn validate_monsters(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("bestiary");
    let mut report = SectionReport::new("monsters");

    if !path.exists() {
        report.warn(None, "bestiary directory not found");
        return report;
    }

    for (_filename, path) in json_files_with_prefix(&path, "bestiary-") {
        let Some(file) = read_json_file::<MonsterFile>(&path, &mut report) else {
            continue;
        };

        for raw in file.monster {
            report.note_seen();
            validate_optional(&mut report, |report| normalize_monster(raw, report));
        }
    }

    info!(count = report.imported, "validated monsters");
    report
}

fn validate_items(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("items.json");
    let mut report = SectionReport::new("items");

    if !path.exists() {
        report.warn(None, "items.json not found");
        return report;
    }

    let Some(file) = read_json_file::<super::types::ItemFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.item {
        report.note_seen();
        validate_optional(&mut report, |report| {
            super::normalize::normalize_item(raw, report)
        });
    }

    info!(count = report.imported, "validated items");
    report
}

fn validate_classes(data_dir: &Path) -> SectionReport {
    validate_class_family(data_dir, "classes", |file, report| {
        for raw in file.classes {
            report.note_seen();
            validate_infallible(report, |report| normalize_class(raw, report));
        }
    })
}

fn validate_subclasses(data_dir: &Path) -> SectionReport {
    validate_class_family(data_dir, "subclasses", |file, report| {
        for raw in file.subclasses {
            report.note_seen();
            validate_infallible(report, |report| normalize_subclass(raw, report));
        }
    })
}

fn validate_class_features(data_dir: &Path) -> SectionReport {
    validate_class_family(data_dir, "class_features", |file, report| {
        for raw in file.class_features {
            report.note_seen();
            validate_infallible(report, |report| normalize_class_feature(raw, report));
        }
    })
}

fn validate_subclass_features(data_dir: &Path) -> SectionReport {
    validate_class_family(data_dir, "subclass_features", |file, report| {
        for raw in file.subclass_features {
            report.note_seen();
            validate_infallible(report, |report| normalize_subclass_feature(raw, report));
        }
    })
}

fn validate_actions(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("actions.json");
    let mut report = SectionReport::new("actions");

    if !path.exists() {
        report.warn(None, "actions.json not found");
        return report;
    }

    let Some(file) = read_json_file::<ActionFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.action {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_action(raw, report));
    }

    info!(count = report.imported, "validated actions");
    report
}

fn validate_feats(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("feats.json");
    let mut report = SectionReport::new("feats");

    if !path.exists() {
        report.warn(None, "feats.json not found");
        return report;
    }

    let Some(file) = read_json_file::<FeatFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.feat {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_feat(raw, report));
    }

    info!(count = report.imported, "validated feats");
    report
}

fn validate_conditions(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("conditionsdiseases.json");
    let mut report = SectionReport::new("conditions");

    if !path.exists() {
        report.warn(None, "conditionsdiseases.json not found");
        return report;
    }

    let Some(file) = read_json_file::<ConditionFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.condition.into_iter().chain(file.disease) {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_condition(raw, report));
    }

    info!(count = report.imported, "validated conditions");
    report
}

fn validate_backgrounds(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("backgrounds.json");
    let mut report = SectionReport::new("backgrounds");

    if !path.exists() {
        report.warn(None, "backgrounds.json not found");
        return report;
    }

    let Some(file) = read_json_file::<BackgroundFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.background {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_background(raw, report));
    }

    info!(count = report.imported, "validated backgrounds");
    report
}

fn validate_races(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("races.json");
    let mut report = SectionReport::new("races");

    if !path.exists() {
        report.warn(None, "races.json not found");
        return report;
    }

    let Some(file) = read_json_file::<RaceFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.race {
        report.note_seen();
        validate_optional(&mut report, |report| normalize_race(raw, report));
    }

    info!(count = report.imported, "validated races");
    report
}

fn validate_optional_features(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("optionalfeatures.json");
    let mut report = SectionReport::new("optional_features");

    if !path.exists() {
        report.warn(None, "optionalfeatures.json not found");
        return report;
    }

    let Some(file) = read_json_file::<OptionalFeatureFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.optionalfeature {
        report.note_seen();
        validate_infallible(&mut report, |report| {
            normalize_optional_feature(raw, report)
        });
    }

    info!(count = report.imported, "validated optional features");
    report
}

fn validate_languages(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("languages.json");
    let mut report = SectionReport::new("languages");

    if !path.exists() {
        report.warn(None, "languages.json not found");
        return report;
    }

    let Some(file) = read_json_file::<LanguageFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.language {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_language(raw, report));
    }

    info!(count = report.imported, "validated languages");
    report
}

fn validate_senses(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("senses.json");
    let mut report = SectionReport::new("senses");

    if !path.exists() {
        report.warn(None, "senses.json not found");
        return report;
    }

    let Some(file) = read_json_file::<SenseFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.sense {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_sense(raw, report));
    }

    info!(count = report.imported, "validated senses");
    report
}

fn validate_skills(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("skills.json");
    let mut report = SectionReport::new("skills");

    if !path.exists() {
        report.warn(None, "skills.json not found");
        return report;
    }

    let Some(file) = read_json_file::<SkillFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.skill {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_skill(raw, report));
    }

    info!(count = report.imported, "validated skills");
    report
}

fn validate_objects(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("objects.json");
    let mut report = SectionReport::new("objects");

    if !path.exists() {
        report.warn(None, "objects.json not found");
        return report;
    }

    let Some(file) = read_json_file::<ObjectFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.object {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_object(raw, report));
    }

    info!(count = report.imported, "validated objects");
    report
}

fn validate_vehicles(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("vehicles.json");
    let mut report = SectionReport::new("vehicles");

    if !path.exists() {
        report.warn(None, "vehicles.json not found");
        return report;
    }

    let Some(file) = read_json_file::<VehicleFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.vehicle {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_vehicle(raw, report));
    }

    info!(count = report.imported, "validated vehicles");
    report
}

fn validate_deities(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("deities.json");
    let mut report = SectionReport::new("deities");

    if !path.exists() {
        report.warn(None, "deities.json not found");
        return report;
    }

    let Some(file) = read_json_file::<DeityFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.deity {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_deity(raw, report));
    }

    info!(count = report.imported, "validated deities");
    report
}

fn validate_rewards(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("rewards.json");
    let mut report = SectionReport::new("rewards");

    if !path.exists() {
        report.warn(None, "rewards.json not found");
        return report;
    }

    let Some(file) = read_json_file::<RewardFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.reward {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_reward(raw, report));
    }

    info!(count = report.imported, "validated rewards");
    report
}

fn validate_trap_hazards(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("trapshazards.json");
    let mut report = SectionReport::new("trap_hazards");

    if !path.exists() {
        report.warn(None, "trapshazards.json not found");
        return report;
    }

    let Some(file) = read_json_file::<TrapHazardFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.trap {
        report.note_seen();
        validate_infallible(&mut report, |report| {
            normalize_trap_hazard(raw, "trap", report)
        });
    }

    for raw in file.hazard {
        report.note_seen();
        validate_infallible(&mut report, |report| {
            normalize_trap_hazard(raw, "hazard", report)
        });
    }

    info!(count = report.imported, "validated trap hazards");
    report
}

fn validate_char_creation_options(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("charcreationoptions.json");
    let mut report = SectionReport::new("char_creation_options");

    if !path.exists() {
        report.warn(None, "charcreationoptions.json not found");
        return report;
    }

    let Some(file) = read_json_file::<CharCreationOptionFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.charoption {
        report.note_seen();
        validate_infallible(&mut report, |report| {
            normalize_char_creation_option(raw, report)
        });
    }

    info!(count = report.imported, "validated char creation options");
    report
}

fn validate_psionics(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("psionics.json");
    let mut report = SectionReport::new("psionics");

    if !path.exists() {
        report.warn(None, "psionics.json not found");
        return report;
    }

    let Some(file) = read_json_file::<PsionicFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.psionic {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_psionic(raw, report));
    }

    info!(count = report.imported, "validated psionics");
    report
}

fn validate_recipes(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("recipes.json");
    let mut report = SectionReport::new("recipes");

    if !path.exists() {
        report.warn(None, "recipes.json not found");
        return report;
    }

    let Some(file) = read_json_file::<RecipeFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.recipe {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_recipe(raw, report));
    }

    info!(count = report.imported, "validated recipes");
    report
}

fn validate_cults_boons(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("cultsboons.json");
    let mut report = SectionReport::new("cults_boons");

    if !path.exists() {
        report.warn(None, "cultsboons.json not found");
        return report;
    }

    let Some(file) = read_json_file::<CultsBoonsFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.cult {
        report.note_seen();
        validate_infallible(&mut report, |report| {
            normalize_cult_boon(raw, "cult", report)
        });
    }

    for raw in file.boon {
        report.note_seen();
        validate_infallible(&mut report, |report| {
            normalize_cult_boon(raw, "boon", report)
        });
    }

    info!(count = report.imported, "validated cults and boons");
    report
}

fn validate_decks(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("decks.json");
    let mut report = SectionReport::new("decks");

    if !path.exists() {
        report.warn(None, "decks.json not found");
        return report;
    }

    let Some(file) = read_json_file::<DeckFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.deck {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_deck(raw, report));
    }

    info!(count = report.imported, "validated decks");
    report
}

fn validate_variant_rules(data_dir: &Path) -> SectionReport {
    let path = data_dir.join("variantrules.json");
    let mut report = SectionReport::new("variant_rules");

    if !path.exists() {
        report.warn(None, "variantrules.json not found");
        return report;
    }

    let Some(file) = read_json_file::<VariantRuleFile>(&path, &mut report) else {
        return report;
    };

    for raw in file.variantrule {
        report.note_seen();
        validate_infallible(&mut report, |report| normalize_variant_rule(raw, report));
    }

    info!(count = report.imported, "validated variant rules");
    report
}

fn validate_class_family(
    data_dir: &Path,
    section_name: &'static str,
    mut process: impl FnMut(ClassFile, &mut SectionReport),
) -> SectionReport {
    let path = data_dir.join("class");
    let mut report = SectionReport::new(section_name);

    if !path.exists() {
        report.warn(None, "class directory not found");
        return report;
    }

    for (filename, path) in json_files_with_prefix(&path, "class-") {
        if filename == "class-sidekick.json" {
            warn_skipped_class_sidekick(&mut report, section_name);
            continue;
        }
        let Some(file) = read_json_file::<ClassFile>(&path, &mut report) else {
            continue;
        };
        process(file, &mut report);
    }

    info!(
        count = report.imported,
        section = section_name,
        "validated class family"
    );
    report
}

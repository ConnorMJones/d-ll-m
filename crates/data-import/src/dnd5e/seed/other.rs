use super::support::read_json_file;
use crate::dnd5e::normalize::{
    normalize_action, normalize_background, normalize_condition, normalize_feat,
    normalize_language, normalize_optional_feature, normalize_race, normalize_sense,
    normalize_skill,
};
use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::{
    ActionFile, BackgroundFile, ConditionFile, FeatFile, LanguageFile, OptionalFeatureFile,
    RaceFile, SenseFile, SkillFile,
};
use crate::dnd5e::write;
use dllm_client::DbConnection;
use std::path::Path;
use tracing::info;

pub fn seed_actions(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("actions.json");
    let mut report = SectionReport::new("actions");

    if !path.exists() {
        info!("actions.json not found, skipping");
        return report;
    }

    info!("reading actions");

    let Some(file) = read_json_file::<ActionFile>(&path, &mut report) else {
        return report;
    };

    for raw_action in file.action {
        report.note_seen();
        let action = normalize_action(raw_action, &mut report);
        let item_name = format!("{} [{}]", action.name, action.source);

        match write::action(conn, action) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded actions");
    report
}

pub fn seed_feats(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("feats.json");
    let mut report = SectionReport::new("feats");

    if !path.exists() {
        info!("feats.json not found, skipping");
        return report;
    }

    info!("reading feats");

    let Some(feat_file) = read_json_file::<FeatFile>(&path, &mut report) else {
        return report;
    };

    for raw_feat in feat_file.feat {
        report.note_seen();
        let feat = normalize_feat(raw_feat, &mut report);
        let item_name = format!("{} [{}]", feat.name, feat.source);

        match write::feat(conn, feat) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded feats");
    report
}

pub fn seed_conditions(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("conditionsdiseases.json");
    let mut report = SectionReport::new("conditions");

    if !path.exists() {
        info!("conditionsdiseases.json not found, skipping");
        return report;
    }

    info!("reading conditions");

    let Some(file) = read_json_file::<ConditionFile>(&path, &mut report) else {
        return report;
    };

    for raw_condition in file.condition.into_iter().chain(file.disease) {
        report.note_seen();
        let condition = normalize_condition(raw_condition, &mut report);
        let item_name = format!("{} [{}]", condition.name, condition.source);

        match write::condition(conn, condition) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded conditions");
    report
}

pub fn seed_backgrounds(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("backgrounds.json");
    let mut report = SectionReport::new("backgrounds");

    if !path.exists() {
        info!("backgrounds.json not found, skipping");
        return report;
    }

    info!("reading backgrounds");

    let Some(file) = read_json_file::<BackgroundFile>(&path, &mut report) else {
        return report;
    };

    for raw_bg in file.background {
        report.note_seen();
        let bg = normalize_background(raw_bg, &mut report);
        let item_name = format!("{} [{}]", bg.name, bg.source);
        match write::background(conn, bg) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded backgrounds");
    report
}

pub fn seed_races(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("races.json");
    let mut report = SectionReport::new("races");

    if !path.exists() {
        info!("races.json not found, skipping");
        return report;
    }

    info!("reading races");

    let Some(file) = read_json_file::<RaceFile>(&path, &mut report) else {
        return report;
    };

    for raw_race in file.race {
        report.note_seen();
        let race = match normalize_race(raw_race, &mut report) {
            Some(race) => race,
            None => continue,
        };
        let item_name = format!("{} [{}]", race.name, race.source);
        match write::race(conn, race) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded races");
    report
}

pub fn seed_optional_features(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("optionalfeatures.json");
    let mut report = SectionReport::new("optional_features");

    if !path.exists() {
        info!("optionalfeatures.json not found, skipping");
        return report;
    }

    info!("reading optional features");

    let Some(file) = read_json_file::<OptionalFeatureFile>(&path, &mut report) else {
        return report;
    };

    for raw_feature in file.optionalfeature {
        report.note_seen();
        let feature = normalize_optional_feature(raw_feature, &mut report);
        let item_name = format!("{} [{}]", feature.name, feature.source);
        match write::optional_feature(conn, feature) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded optional features");
    report
}

pub fn seed_languages(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("languages.json");
    let mut report = SectionReport::new("languages");

    if !path.exists() {
        info!("languages.json not found, skipping");
        return report;
    }

    info!("reading languages");

    let Some(file) = read_json_file::<LanguageFile>(&path, &mut report) else {
        return report;
    };

    for raw_language in file.language {
        report.note_seen();
        let language = normalize_language(raw_language, &mut report);
        let item_name = format!("{} [{}]", language.name, language.source);

        match write::language(conn, language) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded languages");
    report
}

pub fn seed_senses(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("senses.json");
    let mut report = SectionReport::new("senses");

    if !path.exists() {
        info!("senses.json not found, skipping");
        return report;
    }

    info!("reading senses");

    let Some(file) = read_json_file::<SenseFile>(&path, &mut report) else {
        return report;
    };

    for raw_sense in file.sense {
        report.note_seen();
        let sense = normalize_sense(raw_sense, &mut report);
        let item_name = format!("{} [{}]", sense.name, sense.source);

        match write::sense(conn, sense) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded senses");
    report
}

pub fn seed_skills(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("skills.json");
    let mut report = SectionReport::new("skills");

    if !path.exists() {
        info!("skills.json not found, skipping");
        return report;
    }

    info!("reading skills");

    let Some(file) = read_json_file::<SkillFile>(&path, &mut report) else {
        return report;
    };

    for raw_skill in file.skill {
        report.note_seen();
        let skill = normalize_skill(raw_skill, &mut report);
        let item_name = format!("{} [{}]", skill.name, skill.source);

        match write::skill(conn, skill) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded skills");
    report
}

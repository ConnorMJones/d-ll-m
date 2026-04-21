use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::*;
use dllm_core::dnd5e as dnd;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use tracing::{error, info};

pub fn read_json_file<T: DeserializeOwned>(path: &Path, report: &mut SectionReport) -> Option<T> {
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    match serde_json::from_str(&content) {
        Ok(value) => Some(value),
        Err(err) => {
            let item = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned());
            error!(path = ?path, %err, "failed to parse");
            report.warn(item, format!("failed to parse file: {err}"));
            report.failed += 1;
            None
        }
    }
}

pub fn json_files_with_prefix(dir: &Path, prefix: &str) -> Vec<(String, PathBuf)> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if !path.extension().is_some_and(|ext| ext == "json") {
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        if !filename.starts_with(prefix) {
            continue;
        }

        info!(filename, "processing");
        files.push((filename, path));
    }

    files
}

pub fn warn_skipped_class_sidekick(report: &mut SectionReport, context: &'static str) {
    report.warn(
        Some("class-sidekick.json".to_string()),
        format!("skipping sidekick data for {context}; current importer only covers core class-family content"),
    );
}

pub fn parse_skill_grants(
    item_name: &str,
    profs: &[RawSkillBlock],
    report: &mut SectionReport,
) -> Vec<dnd::SkillGrant> {
    profs
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if let Some(ref choose) = block.choose {
                Some(dnd::SkillGrant::Choose(dnd::SkillChoice {
                    count: choose.count.unwrap_or(1),
                    from: choose.from.clone(),
                }))
            } else if let Some(n) = block.any {
                Some(dnd::SkillGrant::Any(n))
            } else {
                let skills = block.to_fixed_skills();
                if skills.is_empty() {
                    report.warn(
                        Some(item_name.to_string()),
                        format!(
                            "skill proficiency block {} contained no supported grant shape",
                            index + 1
                        ),
                    );
                    None
                } else {
                    Some(dnd::SkillGrant::Fixed(skills))
                }
            }
        })
        .collect()
}

pub fn parse_tool_grants(
    item_name: &str,
    profs: &[RawToolBlock],
    report: &mut SectionReport,
) -> Vec<dnd::ToolGrant> {
    profs
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if let Some(ref choose) = block.choose {
                Some(dnd::ToolGrant::Choose(dnd::StringChoice {
                    count: choose.count.unwrap_or(1),
                    from: choose.from.clone(),
                }))
            } else if let Some(n) = block.any {
                Some(dnd::ToolGrant::Any(n))
            } else {
                let tools: Vec<String> = block
                    .tools
                    .iter()
                    .filter(|(_, v)| v.is_truthy())
                    .map(|(k, _)| k.clone())
                    .collect();
                if tools.is_empty() {
                    report.warn(
                        Some(item_name.to_string()),
                        format!(
                            "tool proficiency block {} contained no supported grant shape",
                            index + 1
                        ),
                    );
                    None
                } else {
                    Some(dnd::ToolGrant::Fixed(tools))
                }
            }
        })
        .collect()
}

pub fn parse_language_grants(
    item_name: &str,
    profs: &[RawLanguageBlock],
    report: &mut SectionReport,
) -> Vec<dnd::LanguageGrant> {
    profs
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if let Some(n) = block.any_standard {
                Some(dnd::LanguageGrant::AnyStandard(n))
            } else if let Some(n) = block.any_exotic {
                Some(dnd::LanguageGrant::AnyExotic(n))
            } else if let Some(n) = block.any {
                Some(dnd::LanguageGrant::Any(n))
            } else if let Some(ref choose) = block.choose {
                Some(dnd::LanguageGrant::Choose(dnd::LanguageChoice {
                    count: choose.count.unwrap_or(1),
                    from: choose.from.clone(),
                }))
            } else {
                let langs = block.to_fixed_languages();
                if langs.is_empty() {
                    report.warn(
                        Some(item_name.to_string()),
                        format!(
                            "language proficiency block {} contained no supported grant shape",
                            index + 1
                        ),
                    );
                    None
                } else {
                    Some(dnd::LanguageGrant::Fixed(langs))
                }
            }
        })
        .collect()
}

pub fn parse_ability_grants(
    item_name: &str,
    blocks: &[RawAbilityBlock],
    report: &mut SectionReport,
) -> Vec<dnd::AbilityGrant> {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if let Some(ref choose) = block.choose {
                Some(dnd::AbilityGrant::ChooseAny(dnd::AbilityChoice {
                    count: choose.count,
                    amount: choose.amount.unwrap_or(1),
                }))
            } else {
                let bonuses: Vec<_> = [
                    (dnd::Ability::Strength, block.str_bonus),
                    (dnd::Ability::Dexterity, block.dex_bonus),
                    (dnd::Ability::Constitution, block.con_bonus),
                    (dnd::Ability::Intelligence, block.int_bonus),
                    (dnd::Ability::Wisdom, block.wis_bonus),
                    (dnd::Ability::Charisma, block.cha_bonus),
                ]
                .into_iter()
                .filter_map(|(ability, bonus)| {
                    bonus.map(|b| dnd::AbilityBonus { ability, bonus: b })
                })
                .collect();

                if bonuses.is_empty() {
                    report.warn(
                        Some(item_name.to_string()),
                        format!(
                            "ability grant block {} contained no supported grant shape",
                            index + 1
                        ),
                    );
                    None
                } else {
                    Some(dnd::AbilityGrant::Fixed(bonuses))
                }
            }
        })
        .collect()
}

pub fn parse_race_speed(speed: &Option<RawRaceSpeedValue>) -> dnd::Speed {
    match speed {
        Some(value) => {
            let s = value.as_speed();
            let walk = s.walk.unwrap_or(30);
            dnd::Speed {
                walk,
                fly: s.fly.as_ref().map(|e| e.resolve(walk)).unwrap_or(0),
                swim: s.swim.as_ref().map(|e| e.resolve(walk)).unwrap_or(0),
                climb: s.climb.as_ref().map(|e| e.resolve(walk)).unwrap_or(0),
                burrow: s.burrow.as_ref().map(|e| e.resolve(walk)).unwrap_or(0),
            }
        }
        None => dnd::Speed::default(),
    }
}

pub fn parse_feat_prereq(
    prereqs: &Option<Vec<RawFeatPrereq>>,
    item_name: &str,
    report: &mut SectionReport,
) -> Option<dnd::FeatPrereq> {
    let prereqs = prereqs.as_ref()?;
    if prereqs.len() > 1 {
        report.warn(
            Some(item_name.to_string()),
            format!(
                "multiple prerequisite blocks ({}) truncated to the first block",
                prereqs.len()
            ),
        );
    }
    let prereq = prereqs.first()?;

    let race_refs = prereq.race.as_deref().unwrap_or_default();

    Some(dnd::FeatPrereq {
        level: prereq.level.as_ref().map(|level| level.get()),
        races: race_refs.iter().filter_map(|r| r.as_race()).collect(),
        sizes: race_refs.iter().filter_map(|r| r.as_size()).collect(),
        abilities: prereq
            .ability
            .as_ref()
            .map(|abs| abs.iter().flat_map(|a| a.to_ability_scores()).collect())
            .unwrap_or_default(),
        spellcasting: prereq.spellcasting.unwrap_or(false),
    })
}

pub fn parse_optional_feature_prereq(
    prereqs: &Option<Vec<RawOptionalFeaturePrereq>>,
    item_name: &str,
    report: &mut SectionReport,
) -> Option<dnd::OptionalFeaturePrereq> {
    let prereqs = prereqs.as_ref()?;
    if prereqs.len() > 1 {
        report.warn(
            Some(item_name.to_string()),
            format!(
                "multiple prerequisite blocks ({}) truncated to the first block",
                prereqs.len()
            ),
        );
    }
    let prereq = prereqs.first()?;
    Some(dnd::OptionalFeaturePrereq {
        level: prereq.level.as_ref().map(|l| dnd::ClassLevelPrereq {
            class: l.class.name,
            level: l.level,
        }),
        pact: prereq.pact,
        patron: prereq.patron,
    })
}

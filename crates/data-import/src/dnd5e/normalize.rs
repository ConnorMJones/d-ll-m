use super::entry::{Entry, entries_to_string};
use super::report::SectionReport;
use super::seed::support::{
    parse_ability_grants, parse_feat_prereq, parse_language_grants, parse_optional_feature_prereq,
    parse_race_speed, parse_skill_grants, parse_tool_grants,
};
use super::types::{
    RawAction, RawActionTime, RawBackground, RawCondition, RawFeat, RawItem, RawLanguage,
    RawMonster, RawOptionalFeature, RawRace, RawSense, RawSkill, RawSpell,
};
use dllm::dnd5e as dnd;

#[derive(Debug, Clone)]
pub struct ImportSpell {
    pub name: String,
    pub source: String,
    pub level: u8,
    pub school: dnd::SpellSchool,
    pub ritual: bool,
    pub concentration: bool,
    pub description: String,
    pub saving_throw: Option<dnd::Ability>,
}

#[derive(Debug, Clone)]
pub struct ImportMonster {
    pub name: String,
    pub source: String,
    pub size: dnd::CreatureSize,
    pub creature_type: dnd::CreatureType,
    pub cr: String,
    pub ac: u8,
    pub hp_average: u16,
    pub hp_formula: String,
    pub speed_walk: u16,
    pub speed_fly: u16,
    pub speed_swim: u16,
    pub str_score: u8,
    pub dex_score: u8,
    pub con_score: u8,
    pub int_score: u8,
    pub wis_score: u8,
    pub cha_score: u8,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportItem {
    pub name: String,
    pub source: String,
    pub item_type: dnd::ItemType,
    pub rarity: dnd::ItemRarity,
    pub weight: Option<f32>,
    pub value: Option<u32>,
    pub wondrous: bool,
    pub attunement: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportFeat {
    pub name: String,
    pub source: String,
    pub category: Option<dnd::FeatCategory>,
    pub prerequisite: Option<dnd::FeatPrereq>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportCondition {
    pub name: String,
    pub source: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportBackground {
    pub name: String,
    pub source: String,
    pub skill_grants: Vec<dnd::SkillGrant>,
    pub tool_grants: Vec<dnd::ToolGrant>,
    pub language_grants: Vec<dnd::LanguageGrant>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportRace {
    pub name: String,
    pub source: String,
    pub size: dnd::CreatureSize,
    pub speed: dnd::Speed,
    pub ability_grants: Vec<dnd::AbilityGrant>,
    pub language_grants: Vec<dnd::LanguageGrant>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportOptionalFeature {
    pub name: String,
    pub source: String,
    pub feature_types: Vec<dnd::OptionalFeatureType>,
    pub prerequisite: Option<dnd::OptionalFeaturePrereq>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportAction {
    pub name: String,
    pub source: String,
    pub time: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportLanguage {
    pub name: String,
    pub source: String,
    pub kind: Option<String>,
    pub script: Option<String>,
    pub origin: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportSense {
    pub name: String,
    pub source: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportSkill {
    pub name: String,
    pub source: String,
    pub ability: dnd::Ability,
    pub description: String,
}

pub fn normalize_spell(raw: RawSpell, report: &mut SectionReport) -> ImportSpell {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportSpell {
        name: raw.name,
        source: raw.source,
        level: raw.level,
        school: raw.school,
        ritual: raw.meta.map(|m| m.ritual).unwrap_or(false),
        concentration: raw
            .duration
            .first()
            .map(|d| d.concentration)
            .unwrap_or(false),
        description,
        saving_throw: raw.saving_throw.first().copied(),
    }
}

pub fn normalize_monster(raw: RawMonster, report: &mut SectionReport) -> Option<ImportMonster> {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let size = match raw.size.first() {
        Some(&size) => size,
        None => {
            report.skipped(item_name.clone(), "missing size");
            return None;
        }
    };

    let creature_type = match raw.creature_type {
        Some(creature_type) => creature_type.get(),
        None => {
            report.skipped(item_name.clone(), "missing creature type");
            return None;
        }
    };

    let hp = raw.hp.unwrap_or_default();
    let speed = raw.speed.unwrap_or_default();

    Some(ImportMonster {
        name: raw.name,
        source: raw.source,
        size,
        creature_type,
        cr: raw.cr.as_ref().map(|c| c.get()).unwrap_or_default(),
        ac: raw.ac.first().map(|a| a.get()).unwrap_or(10),
        hp_average: hp.average,
        hp_formula: hp.formula.unwrap_or_default(),
        speed_walk: speed.walk.as_ref().map(|s| s.get()).unwrap_or(0),
        speed_fly: speed.fly.as_ref().map(|s| s.get()).unwrap_or(0),
        speed_swim: speed.swim.as_ref().map(|s| s.get()).unwrap_or(0),
        str_score: raw.str_score,
        dex_score: raw.dex_score,
        con_score: raw.con_score,
        int_score: raw.int_score,
        wis_score: raw.wis_score,
        cha_score: raw.cha_score,
        description: String::new(),
    })
}

pub fn normalize_item(raw: RawItem, report: &mut SectionReport) -> Option<ImportItem> {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let item_type = match raw.item_type.as_ref().and_then(|item_type| item_type.get()) {
        Some(item_type) => item_type,
        None if raw.wondrous => dnd::ItemType::WondrousItem,
        None => {
            report.skipped(item_name.clone(), "unsupported or missing item type");
            return None;
        }
    };

    let description = normalize_description(&item_name, &raw.entries, report);

    Some(ImportItem {
        name: raw.name,
        source: raw.source,
        item_type,
        rarity: raw.rarity.unwrap_or(dnd::ItemRarity::NoRarity),
        weight: raw.weight,
        value: raw.value,
        wondrous: raw.wondrous,
        attunement: raw.req_attune.as_ref().and_then(|a| a.get()),
        description,
    })
}

pub fn normalize_feat(raw: RawFeat, report: &mut SectionReport) -> ImportFeat {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportFeat {
        name: raw.name,
        source: raw.source,
        category: raw.category,
        prerequisite: parse_feat_prereq(&raw.prerequisite, &item_name, report),
        description,
    }
}

pub fn normalize_condition(raw: RawCondition, report: &mut SectionReport) -> ImportCondition {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportCondition {
        name: raw.name,
        source: raw.source,
        description,
    }
}

pub fn normalize_background(raw: RawBackground, report: &mut SectionReport) -> ImportBackground {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportBackground {
        name: raw.name,
        source: raw.source,
        skill_grants: parse_skill_grants(&raw.skill_proficiencies),
        tool_grants: parse_tool_grants(&raw.tool_proficiencies),
        language_grants: parse_language_grants(&raw.language_proficiencies),
        description,
    }
}

pub fn normalize_race(raw: RawRace, report: &mut SectionReport) -> Option<ImportRace> {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let size = match raw.size.first() {
        Some(&size) => size,
        None => {
            report.skipped(item_name.clone(), "missing size");
            return None;
        }
    };

    let description = normalize_description(&item_name, &raw.entries, report);

    Some(ImportRace {
        name: raw.name,
        source: raw.source,
        size,
        speed: parse_race_speed(&raw.speed),
        ability_grants: parse_ability_grants(&raw.ability),
        language_grants: parse_language_grants(&raw.language_proficiencies),
        description,
    })
}

pub fn normalize_optional_feature(
    raw: RawOptionalFeature,
    report: &mut SectionReport,
) -> ImportOptionalFeature {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportOptionalFeature {
        name: raw.name,
        source: raw.source,
        feature_types: raw.feature_type,
        prerequisite: parse_optional_feature_prereq(&raw.prerequisite, &item_name, report),
        description,
    }
}

pub fn normalize_action(raw: RawAction, report: &mut SectionReport) -> ImportAction {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportAction {
        name: raw.name,
        source: raw.source,
        time: format_action_time(&raw.time),
        description,
    }
}

pub fn normalize_language(raw: RawLanguage, report: &mut SectionReport) -> ImportLanguage {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportLanguage {
        name: raw.name,
        source: raw.source,
        kind: raw.kind,
        script: raw.script,
        origin: raw.origin,
        description,
    }
}

pub fn normalize_sense(raw: RawSense, report: &mut SectionReport) -> ImportSense {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportSense {
        name: raw.name,
        source: raw.source,
        description,
    }
}

pub fn normalize_skill(raw: RawSkill, report: &mut SectionReport) -> ImportSkill {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportSkill {
        name: raw.name,
        source: raw.source,
        ability: raw.ability,
        description,
    }
}

fn normalize_description(item_name: &str, entries: &[Entry], report: &mut SectionReport) -> String {
    let (description, unsupported_entries) = entries_to_string(entries);
    if unsupported_entries > 0 {
        report.warn(
            Some(item_name.to_string()),
            format!("description contained {unsupported_entries} unsupported entry variant(s)"),
        );
    }
    description
}

fn format_action_time(times: &[RawActionTime]) -> String {
    if times.is_empty() {
        return String::new();
    }

    times
        .iter()
        .map(|time| match time {
            RawActionTime::Structured { number, unit } => format!("{number} {unit}"),
            RawActionTime::Text(text) => text.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

use super::entry::{Entry, entries_to_string};
use super::report::SectionReport;
use super::seed::support::{
    parse_ability_grants, parse_feat_prereq, parse_language_grants, parse_optional_feature_prereq,
    parse_race_speed, parse_skill_grants, parse_tool_grants,
};
use super::types::{
    RawAction, RawActionTime, RawBackground, RawCharCreationOption, RawClass, RawClassFeature,
    RawCondition, RawCultBoon, RawDeck, RawDeity, RawFeat, RawItem, RawLanguage, RawMonster,
    RawObject, RawOptionalFeature, RawPsionic, RawPsionicMode, RawRace, RawRecipe, RawReward,
    RawSense, RawSkill, RawSpell, RawSubclass, RawSubclassFeature, RawTrapHazard, RawVariantRule,
    RawVehicle,
};
use dllm_core::dnd5e as dnd;

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

#[derive(Debug, Clone)]
pub struct ImportClass {
    pub name: String,
    pub source: String,
    pub edition: Option<String>,
    pub hit_die: u8,
    pub saving_throws: Vec<dnd::Ability>,
    pub spellcasting_ability: Option<dnd::Ability>,
    pub caster_progression: Option<dnd::CasterProgression>,
    pub prepared_spells_formula: Option<String>,
    pub prepared_spells_progression: Vec<u8>,
    pub cantrip_progression: Vec<u8>,
    pub class_features: Vec<String>,
    pub subclass_title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImportSubclass {
    pub name: String,
    pub short_name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub edition: Option<String>,
    pub spellcasting_ability: Option<dnd::Ability>,
    pub caster_progression: Option<dnd::CasterProgression>,
    pub cantrip_progression: Vec<u8>,
    pub subclass_features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImportClassFeature {
    pub name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub level: u8,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportSubclassFeature {
    pub name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub subclass_short_name: String,
    pub subclass_source: String,
    pub level: u8,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportObject {
    pub name: String,
    pub source: String,
    pub size: Vec<dnd::CreatureSize>,
    pub object_type: Option<String>,
    pub ac: Option<u16>,
    pub hp: Option<u16>,
    pub description: String,
    pub action_description: String,
}

#[derive(Debug, Clone)]
pub struct ImportVehicle {
    pub name: String,
    pub source: String,
    pub vehicle_type: String,
    pub size: Vec<String>,
    pub terrain: Vec<String>,
    pub crew_capacity: Option<u16>,
    pub passenger_capacity: Option<u16>,
    pub pace: Option<u16>,
    pub ac: Option<u16>,
    pub hp: Option<u16>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportDeity {
    pub name: String,
    pub source: String,
    pub pantheon: Option<String>,
    pub alignment: Vec<dnd::Alignment>,
    pub category: Option<String>,
    pub domains: Vec<String>,
    pub province: Option<String>,
    pub title: Option<String>,
    pub symbol: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportReward {
    pub name: String,
    pub source: String,
    pub kind: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportTrapHazard {
    pub name: String,
    pub source: String,
    pub kind: String,
    pub trap_hazard_type: Option<String>,
    pub trigger: String,
    pub effect: String,
    pub countermeasures: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportCharCreationOption {
    pub name: String,
    pub source: String,
    pub option_types: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportPsionic {
    pub name: String,
    pub source: String,
    pub kind: String,
    pub order: Option<String>,
    pub focus: Option<String>,
    pub description: String,
    pub modes: String,
}

#[derive(Debug, Clone)]
pub struct ImportRecipe {
    pub name: String,
    pub source: String,
    pub kind: Option<String>,
    pub dish_types: Vec<String>,
    pub diet: Option<String>,
    pub serves: Option<String>,
    pub ingredients: String,
    pub instructions: String,
}

#[derive(Debug, Clone)]
pub struct ImportCultBoon {
    pub name: String,
    pub source: String,
    pub kind: String,
    pub subtype: Option<String>,
    pub goal: Option<String>,
    pub cultists: Option<String>,
    pub signature_spells: Option<String>,
    pub ability_text: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportDeck {
    pub name: String,
    pub source: String,
    pub cards: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ImportVariantRule {
    pub name: String,
    pub source: String,
    pub rule_type: Option<String>,
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

    let creature_type = match raw.creature_type.and_then(|t| t.get()) {
        Some(creature_type) => creature_type,
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
    let item_type = match raw.item_type {
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
        value: raw.value.map(gp_to_cp),
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

pub fn normalize_class(raw: RawClass, _report: &mut SectionReport) -> ImportClass {
    ImportClass {
        name: raw.name,
        source: raw.source,
        edition: raw.edition,
        hit_die: raw.hd.faces,
        saving_throws: raw.saving_throws,
        spellcasting_ability: raw.spellcasting_ability,
        caster_progression: raw.caster_progression,
        prepared_spells_formula: raw.prepared_spells_formula,
        prepared_spells_progression: raw.prepared_spells_progression,
        cantrip_progression: raw.cantrip_progression,
        class_features: raw
            .class_features
            .into_iter()
            .map(|feature| feature.reference().to_string())
            .collect(),
        subclass_title: raw.subclass_title,
    }
}

pub fn normalize_subclass(raw: RawSubclass, report: &mut SectionReport) -> ImportSubclass {
    let item_name = format!("{} ({}) [{}]", raw.name, raw.class_name, raw.source);
    if raw.subclass_features.is_empty() {
        report.warn(
            Some(item_name),
            "subclass had no explicit feature references; likely copy-derived data",
        );
    }

    ImportSubclass {
        name: raw.name,
        short_name: raw.short_name,
        source: raw.source,
        class_name: raw.class_name,
        class_source: raw.class_source,
        edition: raw.edition,
        spellcasting_ability: raw.spellcasting_ability,
        caster_progression: raw.caster_progression,
        cantrip_progression: raw.cantrip_progression,
        subclass_features: raw.subclass_features,
    }
}

pub fn normalize_class_feature(
    raw: RawClassFeature,
    report: &mut SectionReport,
) -> ImportClassFeature {
    let item_name = format!("{} ({}) [{}]", raw.name, raw.class_name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportClassFeature {
        name: raw.name,
        source: raw.source,
        class_name: raw.class_name,
        class_source: raw.class_source,
        level: raw.level,
        description,
    }
}

pub fn normalize_subclass_feature(
    raw: RawSubclassFeature,
    report: &mut SectionReport,
) -> ImportSubclassFeature {
    let item_name = format!(
        "{} ({}/{}) [{}]",
        raw.name, raw.class_name, raw.subclass_short_name, raw.source
    );
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportSubclassFeature {
        name: raw.name,
        source: raw.source,
        class_name: raw.class_name,
        class_source: raw.class_source,
        subclass_short_name: raw.subclass_short_name,
        subclass_source: raw.subclass_source,
        level: raw.level,
        description,
    }
}

pub fn normalize_object(raw: RawObject, report: &mut SectionReport) -> ImportObject {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);
    let action_description =
        normalize_named_entries(&item_name, "action entries", &raw.action_entries, report);

    ImportObject {
        name: raw.name,
        source: raw.source,
        size: raw.size,
        object_type: raw.object_type,
        ac: raw.ac.and_then(|ac| ac.get()),
        hp: raw.hp.and_then(|hp| hp.get()),
        description,
        action_description,
    }
}

pub fn normalize_vehicle(raw: RawVehicle, report: &mut SectionReport) -> ImportVehicle {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportVehicle {
        name: raw.name,
        source: raw.source,
        vehicle_type: raw.vehicle_type,
        size: raw.size.map(|size| size.into_vec()).unwrap_or_default(),
        terrain: raw.terrain,
        crew_capacity: raw.cap_crew,
        passenger_capacity: raw.cap_passenger,
        pace: raw.pace.and_then(|pace| pace.get()),
        ac: raw.ac.and_then(|ac| ac.get()),
        hp: raw.hp.and_then(|hp| hp.get()),
        description,
    }
}

pub fn normalize_deity(raw: RawDeity, report: &mut SectionReport) -> ImportDeity {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportDeity {
        name: raw.name,
        source: raw.source,
        pantheon: raw.pantheon,
        alignment: raw.alignment,
        category: raw.category,
        domains: raw.domains,
        province: raw.province,
        title: raw.title,
        symbol: raw.symbol,
        description,
    }
}

pub fn normalize_reward(raw: RawReward, report: &mut SectionReport) -> ImportReward {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportReward {
        name: raw.name,
        source: raw.source,
        kind: raw.kind,
        description,
    }
}

pub fn normalize_trap_hazard(
    raw: RawTrapHazard,
    kind: &str,
    report: &mut SectionReport,
) -> ImportTrapHazard {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportTrapHazard {
        name: raw.name,
        source: raw.source,
        kind: kind.to_string(),
        trap_hazard_type: raw.trap_hazard_type,
        trigger: normalize_named_entries(&item_name, "trigger", &raw.trigger, report),
        effect: normalize_named_entries(&item_name, "effect", &raw.effect, report),
        countermeasures: normalize_named_entries(
            &item_name,
            "countermeasures",
            &raw.countermeasures,
            report,
        ),
        description,
    }
}

pub fn normalize_char_creation_option(
    raw: RawCharCreationOption,
    report: &mut SectionReport,
) -> ImportCharCreationOption {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportCharCreationOption {
        name: raw.name,
        source: raw.source,
        option_types: raw.option_type,
        description,
    }
}

pub fn normalize_psionic(raw: RawPsionic, report: &mut SectionReport) -> ImportPsionic {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportPsionic {
        name: raw.name,
        source: raw.source,
        kind: raw.kind,
        order: raw.order,
        focus: raw.focus,
        description,
        modes: format_psionic_modes(&item_name, &raw.modes, report),
    }
}

pub fn normalize_recipe(raw: RawRecipe, _report: &mut SectionReport) -> ImportRecipe {
    ImportRecipe {
        name: raw.name,
        source: raw.source,
        kind: raw.kind,
        dish_types: raw.dish_types,
        diet: raw.diet,
        serves: format_recipe_serves(raw.serves),
        ingredients: raw
            .ingredients
            .into_iter()
            .map(|ingredient| ingredient.into_text())
            .collect::<Vec<_>>()
            .join("\n"),
        instructions: raw
            .instructions
            .into_iter()
            .map(|instruction| instruction.into_text())
            .collect::<Vec<_>>()
            .join("\n\n"),
    }
}

pub fn normalize_cult_boon(
    raw: RawCultBoon,
    kind: &str,
    report: &mut SectionReport,
) -> ImportCultBoon {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportCultBoon {
        name: raw.name,
        source: raw.source,
        kind: kind.to_string(),
        subtype: raw.kind,
        goal: raw.goal.map(|value| value.entry),
        cultists: raw.cultists.map(|value| value.entry),
        signature_spells: raw.signature_spells.map(|value| value.entry),
        ability_text: raw.ability_text.map(|value| value.entry),
        description,
    }
}

pub fn normalize_deck(raw: RawDeck, report: &mut SectionReport) -> ImportDeck {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportDeck {
        name: raw.name,
        source: raw.source,
        cards: raw
            .cards
            .into_iter()
            .map(|card| card.into_summary())
            .collect(),
        description,
    }
}

pub fn normalize_variant_rule(
    raw: RawVariantRule,
    report: &mut SectionReport,
) -> ImportVariantRule {
    let item_name = format!("{} [{}]", raw.name, raw.source);
    let description = normalize_description(&item_name, &raw.entries, report);

    ImportVariantRule {
        name: raw.name,
        source: raw.source,
        rule_type: raw.rule_type,
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

fn normalize_named_entries(
    item_name: &str,
    section_name: &str,
    entries: &[Entry],
    report: &mut SectionReport,
) -> String {
    let (description, unsupported_entries) = entries_to_string(entries);
    if unsupported_entries > 0 {
        report.warn(
            Some(item_name.to_string()),
            format!("{section_name} contained {unsupported_entries} unsupported entry variant(s)"),
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

fn format_recipe_serves(serves: Option<super::types::RawRecipeServes>) -> Option<String> {
    let serves = serves?;
    let range = match (serves.exact, serves.min, serves.max) {
        (Some(exact), _, _) => exact.to_string(),
        (None, Some(min), Some(max)) => format!("{min}-{max}"),
        (None, Some(min), None) => min.to_string(),
        (None, None, Some(max)) => max.to_string(),
        (None, None, None) => String::new(),
    };

    let mut parts = Vec::new();
    if !range.is_empty() {
        parts.push(range);
    }
    if let Some(note) = serves.note {
        parts.push(note);
    }

    (!parts.is_empty()).then(|| parts.join(" "))
}

fn gp_to_cp(value_gp: f32) -> u32 {
    (value_gp * 100.0).round() as u32
}

fn format_psionic_modes(
    item_name: &str,
    modes: &[RawPsionicMode],
    report: &mut SectionReport,
) -> String {
    modes
        .iter()
        .filter_map(|mode| format_psionic_mode(item_name, mode, report))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_psionic_mode(
    item_name: &str,
    mode: &RawPsionicMode,
    report: &mut SectionReport,
) -> Option<String> {
    let body = normalize_named_entries(item_name, "psionic modes", &mode.entries, report);
    let submodes = mode
        .submodes
        .iter()
        .filter_map(|submode| format_psionic_mode(item_name, submode, report))
        .collect::<Vec<_>>()
        .join("\n\n");

    let mut parts = Vec::new();
    if let Some(name) = mode.name.as_ref() {
        parts.push(name.clone());
    }
    if !body.is_empty() {
        parts.push(body);
    }
    if !submodes.is_empty() {
        parts.push(submodes);
    }

    (!parts.is_empty()).then(|| parts.join("\n"))
}

use crate::convert;
use dllm::dnd5e as dnd;
use dllm_client::{
    DbConnection, dnd_5_e_item_table::Dnd5EItemTableAccess,
    dnd_5_e_monster_table::Dnd5EMonsterTableAccess, dnd_5_e_spell_table::Dnd5ESpellTableAccess,
    item_rarity_type::ItemRarity, item_type_type::ItemType,
};
use serde::Deserialize;
use spacetimedb_sdk::{DbContext, Table};
use std::path::Path;
use tracing::{error, info, warn};

const SPACETIMEDB_URI: &str = "http://127.0.0.1:3033";
const DB_NAME: &str = "dllm";

#[derive(Deserialize)]
struct SpellFile {
    spell: Vec<RawSpell>,
}

#[derive(Deserialize)]
struct RawSpell {
    name: String,
    source: String,
    level: u8,
    school: dnd::SpellSchool,
    #[serde(default)]
    entries: Vec<serde_json::Value>,
    #[serde(default)]
    duration: Vec<DurationEntry>,
    #[serde(default)]
    meta: Option<SpellMeta>,
    #[serde(rename = "savingThrow", default)]
    saving_throw: Vec<dnd::Ability>,
}

#[derive(Deserialize, Default)]
struct SpellMeta {
    #[serde(default)]
    ritual: bool,
}

#[derive(Deserialize)]
struct DurationEntry {
    #[serde(default)]
    concentration: bool,
}

#[derive(Deserialize)]
struct MonsterFile {
    monster: Vec<RawMonster>,
}

#[derive(Deserialize)]
struct RawMonster {
    name: String,
    source: String,
    #[serde(default)]
    size: Vec<dnd::CreatureSize>,
    #[serde(rename = "type", default)]
    creature_type: serde_json::Value,
    #[serde(default)]
    cr: serde_json::Value,
    #[serde(default)]
    ac: Vec<serde_json::Value>,
    #[serde(default)]
    hp: Option<HpValue>,
    #[serde(default)]
    speed: Option<SpeedValue>,
    #[serde(default)]
    str: u8,
    #[serde(default)]
    dex: u8,
    #[serde(default)]
    con: u8,
    #[serde(default)]
    int: u8,
    #[serde(default)]
    wis: u8,
    #[serde(default)]
    cha: u8,
}

#[derive(Deserialize, Default)]
struct HpValue {
    #[serde(default)]
    average: u16,
    #[serde(default)]
    formula: Option<String>,
}

#[derive(Deserialize, Default)]
struct SpeedValue {
    #[serde(default)]
    walk: Option<serde_json::Value>,
    #[serde(default)]
    fly: Option<serde_json::Value>,
    #[serde(default)]
    swim: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct ItemFile {
    #[serde(default)]
    item: Vec<RawItem>,
    #[serde(default)]
    baseitem: Vec<RawItem>,
}

#[derive(Deserialize)]
struct FeatFile {
    feat: Vec<RawFeat>,
}

#[derive(Deserialize)]
struct RawFeat {
    name: String,
    source: String,
    #[serde(default)]
    category: Option<dnd::FeatCategory>,
    #[serde(default)]
    prerequisite: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    entries: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct ConditionFile {
    #[serde(default)]
    condition: Vec<RawCondition>,
    #[serde(default)]
    disease: Vec<RawCondition>,
}

#[derive(Deserialize)]
struct RawCondition {
    name: String,
    source: String,
    #[serde(default)]
    entries: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct BackgroundFile {
    background: Vec<RawBackground>,
}

#[derive(Deserialize)]
struct RaceFile {
    race: Vec<RawRace>,
}

#[derive(Deserialize)]
struct RawBackground {
    name: String,
    source: String,
    #[serde(rename = "skillProficiencies", default)]
    skill_proficiencies: Vec<RawSkillBlock>,
    #[serde(rename = "toolProficiencies", default)]
    tool_proficiencies: Vec<RawToolBlock>,
    #[serde(rename = "languageProficiencies", default)]
    language_proficiencies: Vec<RawLanguageBlock>,
    #[serde(default)]
    entries: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct RawRace {
    name: String,
    source: String,
    #[serde(default)]
    size: Vec<dnd::CreatureSize>,
    #[serde(default)]
    speed: Option<RawRaceSpeed>,
    #[serde(default)]
    ability: Vec<RawAbilityBlock>,
    #[serde(rename = "languageProficiencies", default)]
    language_proficiencies: Vec<RawLanguageBlock>,
    #[serde(default)]
    entries: Vec<serde_json::Value>,
}

#[derive(Deserialize, Default)]
struct RawRaceSpeed {
    #[serde(default)]
    walk: Option<u16>,
    #[serde(default)]
    fly: Option<u16>,
    #[serde(default)]
    swim: Option<u16>,
    #[serde(default)]
    climb: Option<u16>,
    #[serde(default)]
    burrow: Option<u16>,
}

#[derive(Deserialize, Default)]
struct RawAbilityBlock {
    #[serde(default)]
    str: Option<i8>,
    #[serde(default)]
    dex: Option<i8>,
    #[serde(default)]
    con: Option<i8>,
    #[serde(default)]
    int: Option<i8>,
    #[serde(default)]
    wis: Option<i8>,
    #[serde(default)]
    cha: Option<i8>,
    #[serde(default)]
    choose: Option<RawAbilityChoose>,
}

#[derive(Deserialize)]
struct RawAbilityChoose {
    #[serde(default)]
    count: u8,
    #[serde(default)]
    amount: Option<i8>,
}

#[derive(Deserialize, Default)]
struct RawSkillBlock {
    #[serde(default)]
    acrobatics: Option<bool>,
    #[serde(rename = "animal handling", default)]
    animal_handling: Option<bool>,
    #[serde(default)]
    arcana: Option<bool>,
    #[serde(default)]
    athletics: Option<bool>,
    #[serde(default)]
    deception: Option<bool>,
    #[serde(default)]
    history: Option<bool>,
    #[serde(default)]
    insight: Option<bool>,
    #[serde(default)]
    intimidation: Option<bool>,
    #[serde(default)]
    investigation: Option<bool>,
    #[serde(default)]
    medicine: Option<bool>,
    #[serde(default)]
    nature: Option<bool>,
    #[serde(default)]
    perception: Option<bool>,
    #[serde(default)]
    performance: Option<bool>,
    #[serde(default)]
    persuasion: Option<bool>,
    #[serde(default)]
    religion: Option<bool>,
    #[serde(rename = "sleight of hand", default)]
    sleight_of_hand: Option<bool>,
    #[serde(default)]
    stealth: Option<bool>,
    #[serde(default)]
    survival: Option<bool>,
    #[serde(default)]
    any: Option<u8>,
    #[serde(default)]
    choose: Option<RawSkillChoose>,
}

impl RawSkillBlock {
    fn to_fixed_skills(&self) -> Vec<dnd::Skill> {
        let mut skills = Vec::new();
        if self.acrobatics == Some(true) { skills.push(dnd::Skill::Acrobatics); }
        if self.animal_handling == Some(true) { skills.push(dnd::Skill::AnimalHandling); }
        if self.arcana == Some(true) { skills.push(dnd::Skill::Arcana); }
        if self.athletics == Some(true) { skills.push(dnd::Skill::Athletics); }
        if self.deception == Some(true) { skills.push(dnd::Skill::Deception); }
        if self.history == Some(true) { skills.push(dnd::Skill::History); }
        if self.insight == Some(true) { skills.push(dnd::Skill::Insight); }
        if self.intimidation == Some(true) { skills.push(dnd::Skill::Intimidation); }
        if self.investigation == Some(true) { skills.push(dnd::Skill::Investigation); }
        if self.medicine == Some(true) { skills.push(dnd::Skill::Medicine); }
        if self.nature == Some(true) { skills.push(dnd::Skill::Nature); }
        if self.perception == Some(true) { skills.push(dnd::Skill::Perception); }
        if self.performance == Some(true) { skills.push(dnd::Skill::Performance); }
        if self.persuasion == Some(true) { skills.push(dnd::Skill::Persuasion); }
        if self.religion == Some(true) { skills.push(dnd::Skill::Religion); }
        if self.sleight_of_hand == Some(true) { skills.push(dnd::Skill::SleightOfHand); }
        if self.stealth == Some(true) { skills.push(dnd::Skill::Stealth); }
        if self.survival == Some(true) { skills.push(dnd::Skill::Survival); }
        skills
    }
}

#[derive(Deserialize)]
struct RawSkillChoose {
    #[serde(default)]
    from: Vec<String>,
    #[serde(default)]
    count: Option<u8>,
}

#[derive(Deserialize, Default)]
struct RawToolBlock {
    #[serde(default)]
    any: Option<u8>,
    #[serde(default)]
    choose: Option<RawToolChoose>,
    #[serde(flatten)]
    tools: std::collections::HashMap<String, bool>,
}

#[derive(Deserialize)]
struct RawToolChoose {
    #[serde(default)]
    from: Vec<String>,
    #[serde(default)]
    count: Option<u8>,
}

#[derive(Deserialize, Default)]
struct RawLanguageBlock {
    #[serde(default)]
    common: Option<bool>,
    #[serde(default)]
    dwarvish: Option<bool>,
    #[serde(default)]
    elvish: Option<bool>,
    #[serde(default)]
    giant: Option<bool>,
    #[serde(default)]
    gnomish: Option<bool>,
    #[serde(default)]
    goblin: Option<bool>,
    #[serde(default)]
    halfling: Option<bool>,
    #[serde(default)]
    orc: Option<bool>,
    #[serde(default)]
    abyssal: Option<bool>,
    #[serde(default)]
    celestial: Option<bool>,
    #[serde(rename = "deep speech", default)]
    deep_speech: Option<bool>,
    #[serde(default)]
    draconic: Option<bool>,
    #[serde(default)]
    infernal: Option<bool>,
    #[serde(default)]
    primordial: Option<bool>,
    #[serde(default)]
    sylvan: Option<bool>,
    #[serde(default)]
    undercommon: Option<bool>,
    #[serde(default)]
    aquan: Option<bool>,
    #[serde(default)]
    auran: Option<bool>,
    #[serde(default)]
    ignan: Option<bool>,
    #[serde(default)]
    terran: Option<bool>,
    #[serde(default)]
    druidic: Option<bool>,
    #[serde(rename = "thieves' cant", default)]
    thieves_cant: Option<bool>,
    #[serde(default)]
    gith: Option<bool>,
    #[serde(rename = "anyStandard", default)]
    any_standard: Option<u8>,
    #[serde(rename = "anyExotic", default)]
    any_exotic: Option<u8>,
    #[serde(default)]
    any: Option<u8>,
    #[serde(default)]
    choose: Option<RawLanguageChoose>,
}

impl RawLanguageBlock {
    fn to_fixed_languages(&self) -> Vec<dnd::Language> {
        let mut langs = Vec::new();
        if self.common == Some(true) { langs.push(dnd::Language::Common); }
        if self.dwarvish == Some(true) { langs.push(dnd::Language::Dwarvish); }
        if self.elvish == Some(true) { langs.push(dnd::Language::Elvish); }
        if self.giant == Some(true) { langs.push(dnd::Language::Giant); }
        if self.gnomish == Some(true) { langs.push(dnd::Language::Gnomish); }
        if self.goblin == Some(true) { langs.push(dnd::Language::Goblin); }
        if self.halfling == Some(true) { langs.push(dnd::Language::Halfling); }
        if self.orc == Some(true) { langs.push(dnd::Language::Orc); }
        if self.abyssal == Some(true) { langs.push(dnd::Language::Abyssal); }
        if self.celestial == Some(true) { langs.push(dnd::Language::Celestial); }
        if self.deep_speech == Some(true) { langs.push(dnd::Language::DeepSpeech); }
        if self.draconic == Some(true) { langs.push(dnd::Language::Draconic); }
        if self.infernal == Some(true) { langs.push(dnd::Language::Infernal); }
        if self.primordial == Some(true) { langs.push(dnd::Language::Primordial); }
        if self.sylvan == Some(true) { langs.push(dnd::Language::Sylvan); }
        if self.undercommon == Some(true) { langs.push(dnd::Language::Undercommon); }
        if self.aquan == Some(true) { langs.push(dnd::Language::Aquan); }
        if self.auran == Some(true) { langs.push(dnd::Language::Auran); }
        if self.ignan == Some(true) { langs.push(dnd::Language::Ignan); }
        if self.terran == Some(true) { langs.push(dnd::Language::Terran); }
        if self.druidic == Some(true) { langs.push(dnd::Language::Druidic); }
        if self.thieves_cant == Some(true) { langs.push(dnd::Language::ThievesCant); }
        if self.gith == Some(true) { langs.push(dnd::Language::Gith); }
        langs
    }
}

#[derive(Deserialize)]
struct RawLanguageChoose {
    #[serde(default)]
    from: Vec<String>,
    #[serde(default)]
    count: Option<u8>,
}

fn parse_skill_grants(profs: &[RawSkillBlock]) -> Vec<dnd::SkillGrant> {
    profs
        .iter()
        .filter_map(|block| {
            if let Some(ref choose) = block.choose {
                let skills: Vec<dnd::Skill> = choose
                    .from
                    .iter()
                    .filter_map(|s| serde_json::from_value(serde_json::Value::String(s.clone())).ok())
                    .collect();
                Some(dnd::SkillGrant::Choose(dnd::SkillChoice {
                    count: choose.count.unwrap_or(1),
                    from: skills,
                }))
            } else if let Some(n) = block.any {
                Some(dnd::SkillGrant::Any(n))
            } else {
                let skills = block.to_fixed_skills();
                if skills.is_empty() {
                    None
                } else {
                    Some(dnd::SkillGrant::Fixed(skills))
                }
            }
        })
        .collect()
}

fn parse_tool_grants(profs: &[RawToolBlock]) -> Vec<dnd::ToolGrant> {
    profs
        .iter()
        .filter_map(|block| {
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
                    .filter(|(_, &v)| v)
                    .map(|(k, _)| k.clone())
                    .collect();
                if tools.is_empty() {
                    None
                } else {
                    Some(dnd::ToolGrant::Fixed(tools))
                }
            }
        })
        .collect()
}

fn parse_language_grants(profs: &[RawLanguageBlock]) -> Vec<dnd::LanguageGrant> {
    profs
        .iter()
        .filter_map(|block| {
            if let Some(n) = block.any_standard {
                Some(dnd::LanguageGrant::AnyStandard(n))
            } else if let Some(n) = block.any_exotic {
                Some(dnd::LanguageGrant::AnyExotic(n))
            } else if let Some(n) = block.any {
                Some(dnd::LanguageGrant::Any(n))
            } else if let Some(ref choose) = block.choose {
                let langs: Vec<dnd::Language> = choose
                    .from
                    .iter()
                    .filter_map(|s| serde_json::from_value(serde_json::Value::String(s.clone())).ok())
                    .collect();
                Some(dnd::LanguageGrant::Choose(dnd::LanguageChoice {
                    count: choose.count.unwrap_or(1),
                    from: langs,
                }))
            } else {
                let langs = block.to_fixed_languages();
                if langs.is_empty() {
                    None
                } else {
                    Some(dnd::LanguageGrant::Fixed(langs))
                }
            }
        })
        .collect()
}

#[derive(Deserialize)]
struct RawItem {
    name: String,
    source: String,
    #[serde(rename = "type", default)]
    item_type: Option<String>,
    #[serde(default)]
    rarity: Option<String>,
    #[serde(default)]
    weight: Option<f32>,
    #[serde(default)]
    value: Option<u32>,
    #[serde(default)]
    wondrous: bool,
    #[serde(rename = "reqAttune", default)]
    req_attune: Option<serde_json::Value>,
    #[serde(default)]
    entries: Vec<serde_json::Value>,
}

fn entries_to_string(entries: &[serde_json::Value]) -> String {
    entries
        .iter()
        .filter_map(|e| {
            if let serde_json::Value::String(s) = e {
                Some(s.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn parse_speed(val: &Option<serde_json::Value>) -> u16 {
    match val {
        Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0) as u16,
        Some(serde_json::Value::Object(obj)) => {
            obj.get("number").and_then(|n| n.as_u64()).unwrap_or(0) as u16
        }
        _ => 0,
    }
}

fn parse_ac(ac: &[serde_json::Value]) -> u8 {
    ac.first()
        .map(|v| match v {
            serde_json::Value::Number(n) => n.as_u64().unwrap_or(10) as u8,
            serde_json::Value::Object(obj) => {
                obj.get("ac").and_then(|n| n.as_u64()).unwrap_or(10) as u8
            }
            _ => 10,
        })
        .unwrap_or(10)
}

fn parse_cr(cr: &serde_json::Value) -> String {
    match cr {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Object(obj) => obj
            .get("cr")
            .and_then(|v| v.as_str())
            .unwrap_or("0")
            .to_string(),
        _ => "0".to_string(),
    }
}

fn parse_creature_type(ct: &serde_json::Value) -> Option<dnd::CreatureType> {
    let type_str = match ct {
        serde_json::Value::String(s) => Some(s.as_str()),
        serde_json::Value::Object(obj) => obj.get("type").and_then(|v| v.as_str()),
        _ => None,
    }?;
    serde_json::from_value(serde_json::Value::String(type_str.to_string())).ok()
}

fn parse_item_type(code: &str) -> Option<dnd::ItemType> {
    let base_code = code.split('|').next().unwrap_or(code);
    serde_json::from_value(serde_json::Value::String(base_code.to_string())).ok()
}

fn parse_attunement(val: &Option<serde_json::Value>) -> Option<String> {
    match val {
        Some(serde_json::Value::Bool(true)) => Some("requires attunement".to_string()),
        Some(serde_json::Value::String(s)) => Some(format!("requires attunement {}", s)),
        _ => None,
    }
}

fn connect() -> DbConnection {
    info!(uri = SPACETIMEDB_URI, "connecting to SpacetimeDB");

    DbConnection::builder()
        .with_uri(SPACETIMEDB_URI)
        .with_database_name(DB_NAME)
        .on_connect(|_ctx, _identity, _token| {
            info!("connected");
        })
        .on_connect_error(|_ctx, err| {
            error!(?err, "connection failed");
            std::process::exit(1);
        })
        .build()
        .expect("Failed to connect")
}

pub fn query_spells(name_filter: Option<String>, level_filter: Option<u8>) {
    let conn = connect();

    conn.subscription_builder()
        .on_applied(|ctx| {
            let count = ctx.db.dnd_5_e_spell().count();
            info!(count, "subscribed to spells");
        })
        .subscribe(["SELECT * FROM dnd_5_e_spell"]);

    let _handle = conn.run_threaded();
    std::thread::sleep(std::time::Duration::from_secs(2));

    let name_filter = name_filter.map(|n| n.to_lowercase());
    let mut found = 0;

    for spell in conn.db.dnd_5_e_spell().iter() {
        let matches_name = name_filter
            .as_ref()
            .is_none_or(|n| spell.name.to_lowercase().contains(n));
        let matches_level = level_filter.is_none_or(|l| spell.level == l);

        if matches_name && matches_level {
            println!(
                "[{}] {} (Level {}, {:?})",
                spell.source, spell.name, spell.level, spell.school
            );
            if name_filter.is_some() {
                let desc_preview: String = spell.description.chars().take(200).collect();
                println!("  {}", desc_preview);
            }
            found += 1;
        }
    }

    println!("\nFound {} spells.", found);
    conn.disconnect().ok();
}

pub fn query_monsters(name_filter: Option<String>, cr_filter: Option<String>) {
    let conn = connect();

    conn.subscription_builder()
        .on_applied(|ctx| {
            let count = ctx.db.dnd_5_e_monster().count();
            info!(count, "subscribed to monsters");
        })
        .subscribe(["SELECT * FROM dnd_5_e_monster"]);

    let _handle = conn.run_threaded();
    std::thread::sleep(std::time::Duration::from_secs(2));

    let name_filter = name_filter.map(|n| n.to_lowercase());
    let mut found = 0;

    for monster in conn.db.dnd_5_e_monster().iter() {
        let matches_name = name_filter
            .as_ref()
            .is_none_or(|n| monster.name.to_lowercase().contains(n));
        let matches_cr = cr_filter.as_ref().is_none_or(|c| monster.cr == *c);

        if matches_name && matches_cr {
            println!(
                "[{}] {} (CR {}, {:?} {:?}, HP {}, AC {})",
                monster.source,
                monster.name,
                monster.cr,
                monster.size,
                monster.creature_type,
                monster.hp_average,
                monster.ac
            );
            found += 1;
        }
    }

    println!("\nFound {} monsters.", found);
    conn.disconnect().ok();
}

pub fn query_items(name_filter: Option<String>, rarity_filter: Option<String>) {
    let conn = connect();

    conn.subscription_builder()
        .on_applied(|ctx| {
            let count = ctx.db.dnd_5_e_item().count();
            info!(count, "subscribed to items");
        })
        .subscribe(["SELECT * FROM dnd_5_e_item"]);

    let _handle = conn.run_threaded();
    std::thread::sleep(std::time::Duration::from_secs(2));

    let name_filter = name_filter.map(|n| n.to_lowercase());
    let rarity_filter = rarity_filter
        .and_then(|r| serde_json::from_value::<dnd::ItemRarity>(serde_json::Value::String(r)).ok())
        .map(convert::item_rarity);
    let mut found = 0;

    for item in conn.db.dnd_5_e_item().iter() {
        let matches_name = name_filter
            .as_ref()
            .is_none_or(|n| item.name.to_lowercase().contains(n));
        let matches_rarity = rarity_filter.is_none_or(|r| item.rarity == r);

        if matches_name && matches_rarity {
            println!(
                "[{}] {} ({:?}, {:?})",
                item.source, item.name, item.rarity, item.item_type
            );
            found += 1;
        }
    }

    println!("\nFound {} items.", found);
    conn.disconnect().ok();
}

pub fn seed(data_dir: &Path) {
    let conn = connect();

    let _handle = conn.run_threaded();
    std::thread::sleep(std::time::Duration::from_secs(1));

    seed_spells(&conn, data_dir);
    seed_monsters(&conn, data_dir);
    seed_items(&conn, data_dir);
    seed_feats(&conn, data_dir);
    seed_conditions(&conn, data_dir);
    seed_backgrounds(&conn, data_dir);
    seed_races(&conn, data_dir);

    info!("waiting for inserts to complete");
    std::thread::sleep(std::time::Duration::from_secs(5));
    conn.disconnect().ok();

    info!("done");
}

fn seed_spells(conn: &DbConnection, data_dir: &Path) {
    let spells_dir = data_dir.join("spells");
    info!(?spells_dir, "reading spells");

    let mut total = 0;

    for entry in std::fs::read_dir(&spells_dir).expect("Failed to read spells directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if !path.extension().is_some_and(|e| e == "json") {
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();
        if !filename.starts_with("spells-") {
            continue;
        }

        info!(%filename, "processing");

        let content = std::fs::read_to_string(&path).expect("Failed to read file");
        let spell_file: SpellFile = match serde_json::from_str(&content) {
            Ok(f) => f,
            Err(e) => {
                error!(%filename, %e, "failed to parse");
                continue;
            }
        };

        for spell in spell_file.spell {
            let ritual = spell.meta.map(|m| m.ritual).unwrap_or(false);
            let concentration = spell.duration.first().map(|d| d.concentration).unwrap_or(false);
            let description = entries_to_string(&spell.entries);
            let saving_throw = spell.saving_throw.first().copied().map(convert::ability);

            conn.reducers
                .seed_dnd_5_e_spell(
                    spell.name,
                    spell.source,
                    spell.level,
                    convert::spell_school(spell.school),
                    ritual,
                    concentration,
                    description,
                    saving_throw,
                )
                .ok();

            total += 1;
        }
    }

    info!(count = total, "seeded spells");
}

fn seed_monsters(conn: &DbConnection, data_dir: &Path) {
    let bestiary_dir = data_dir.join("bestiary");
    info!(?bestiary_dir, "reading monsters");

    let mut total = 0;

    for entry in std::fs::read_dir(&bestiary_dir).expect("Failed to read bestiary directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if !path.extension().is_some_and(|e| e == "json") {
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();
        if !filename.starts_with("bestiary-") {
            continue;
        }

        info!(%filename, "processing");

        let content = std::fs::read_to_string(&path).expect("Failed to read file");
        let monster_file: MonsterFile = match serde_json::from_str(&content) {
            Ok(f) => f,
            Err(e) => {
                error!(%filename, %e, "failed to parse");
                continue;
            }
        };

        for monster in monster_file.monster {
            let size = match monster.size.first() {
                Some(&s) => convert::creature_size(s),
                None => continue,
            };

            let creature_type = match parse_creature_type(&monster.creature_type) {
                Some(t) => convert::creature_type(t),
                None => continue,
            };

            let hp = monster.hp.unwrap_or_default();
            let speed = monster.speed.unwrap_or_default();

            conn.reducers
                .seed_dnd_5_e_monster(
                    monster.name.clone(),
                    monster.source.clone(),
                    size,
                    creature_type,
                    parse_cr(&monster.cr),
                    parse_ac(&monster.ac),
                    hp.average,
                    hp.formula.unwrap_or_default(),
                    parse_speed(&speed.walk),
                    parse_speed(&speed.fly),
                    parse_speed(&speed.swim),
                    monster.str,
                    monster.dex,
                    monster.con,
                    monster.int,
                    monster.wis,
                    monster.cha,
                    String::new(),
                )
                .ok();

            total += 1;
        }
    }

    info!(count = total, "seeded monsters");
}

fn seed_items(conn: &DbConnection, data_dir: &Path) {
    info!(?data_dir, "reading items");

    let mut total = 0;

    for filename in ["items.json", "items-base.json"] {
        let path = data_dir.join(filename);
        if !path.exists() {
            info!(filename, "skipping (not found)");
            continue;
        }

        info!(filename, "processing");

        let content = std::fs::read_to_string(&path).expect("Failed to read file");
        let item_file: ItemFile = match serde_json::from_str(&content) {
            Ok(f) => f,
            Err(e) => {
                error!(%filename, %e, "failed to parse");
                continue;
            }
        };

        let all_items = item_file.item.into_iter().chain(item_file.baseitem);

        for item in all_items {
            let item_type = match item.item_type.as_deref().and_then(parse_item_type) {
                Some(t) => convert::item_type(t),
                None if item.wondrous => ItemType::WondrousItem,
                None => {
                    if let Some(code) = &item.item_type {
                        warn!(item_type = %code, name = %item.name, "unknown item type");
                    }
                    continue;
                }
            };

            let rarity = item
                .rarity
                .as_ref()
                .and_then(|r| serde_json::from_value::<dnd::ItemRarity>(serde_json::Value::String(r.clone())).ok())
                .map(convert::item_rarity)
                .unwrap_or(ItemRarity::NoRarity);

            let description = entries_to_string(&item.entries);
            let attunement = parse_attunement(&item.req_attune);

            conn.reducers
                .seed_dnd_5_e_item(
                    item.name.clone(),
                    item.source.clone(),
                    item_type,
                    rarity,
                    item.weight,
                    item.value,
                    item.wondrous,
                    attunement,
                    description,
                )
                .ok();

            total += 1;
        }
    }

    info!(count = total, "seeded items");
}

fn seed_feats(conn: &DbConnection, data_dir: &Path) {
    let path = data_dir.join("feats.json");
    if !path.exists() {
        info!("feats.json not found, skipping");
        return;
    }

    info!("reading feats");

    let content = std::fs::read_to_string(&path).expect("Failed to read feats.json");
    let feat_file: FeatFile = match serde_json::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            error!(%e, "failed to parse feats.json");
            return;
        }
    };

    let mut total = 0;

    for feat in feat_file.feat {
        let prerequisite = feat.prerequisite.as_ref().map(|p| format!("{:?}", p));
        let description = entries_to_string(&feat.entries);

        conn.reducers
            .seed_dnd_5_e_feat(
                feat.name,
                feat.source,
                feat.category.map(convert::feat_category),
                prerequisite,
                description,
            )
            .ok();

        total += 1;
    }

    info!(count = total, "seeded feats");
}

fn seed_conditions(conn: &DbConnection, data_dir: &Path) {
    let path = data_dir.join("conditionsdiseases.json");
    if !path.exists() {
        info!("conditionsdiseases.json not found, skipping");
        return;
    }

    info!("reading conditions");

    let content = std::fs::read_to_string(&path).expect("Failed to read conditionsdiseases.json");
    let file: ConditionFile = match serde_json::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            error!(%e, "failed to parse conditionsdiseases.json");
            return;
        }
    };

    let mut total = 0;

    for condition in file.condition.into_iter().chain(file.disease) {
        let description = entries_to_string(&condition.entries);

        conn.reducers
            .seed_dnd_5_e_condition(
                condition.name.clone(),
                condition.source.clone(),
                description,
            )
            .ok();

        total += 1;
    }

    info!(count = total, "seeded conditions");
}

fn seed_backgrounds(conn: &DbConnection, data_dir: &Path) {
    let path = data_dir.join("backgrounds.json");
    if !path.exists() {
        info!("backgrounds.json not found, skipping");
        return;
    }

    info!("reading backgrounds");

    let content = std::fs::read_to_string(&path).expect("Failed to read backgrounds.json");
    let file: BackgroundFile = match serde_json::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            error!(%e, "failed to parse backgrounds.json");
            return;
        }
    };

    let mut total = 0;

    for bg in file.background {
        let skill_grants: Vec<_> = parse_skill_grants(&bg.skill_proficiencies)
            .into_iter()
            .map(convert::skill_grant)
            .collect();
        let tool_grants: Vec<_> = parse_tool_grants(&bg.tool_proficiencies)
            .into_iter()
            .map(convert::tool_grant)
            .collect();
        let lang_grants: Vec<_> = parse_language_grants(&bg.language_proficiencies)
            .into_iter()
            .map(convert::language_grant)
            .collect();
        let description = entries_to_string(&bg.entries);

        conn.reducers
            .seed_dnd_5_e_background(
                bg.name.clone(),
                bg.source.clone(),
                skill_grants,
                tool_grants,
                lang_grants,
                description,
            )
            .ok();

        total += 1;
    }

    info!(count = total, "seeded backgrounds");
}

fn parse_ability_grants(blocks: &[RawAbilityBlock]) -> Vec<dnd::AbilityGrant> {
    blocks
        .iter()
        .filter_map(|block| {
            if let Some(ref choose) = block.choose {
                Some(dnd::AbilityGrant::ChooseAny(dnd::AbilityChoice {
                    count: choose.count,
                    amount: choose.amount.unwrap_or(1),
                }))
            } else {
                let mut bonuses = Vec::new();
                if let Some(v) = block.str { bonuses.push(dnd::AbilityBonus { ability: dnd::Ability::Strength, bonus: v }); }
                if let Some(v) = block.dex { bonuses.push(dnd::AbilityBonus { ability: dnd::Ability::Dexterity, bonus: v }); }
                if let Some(v) = block.con { bonuses.push(dnd::AbilityBonus { ability: dnd::Ability::Constitution, bonus: v }); }
                if let Some(v) = block.int { bonuses.push(dnd::AbilityBonus { ability: dnd::Ability::Intelligence, bonus: v }); }
                if let Some(v) = block.wis { bonuses.push(dnd::AbilityBonus { ability: dnd::Ability::Wisdom, bonus: v }); }
                if let Some(v) = block.cha { bonuses.push(dnd::AbilityBonus { ability: dnd::Ability::Charisma, bonus: v }); }
                if bonuses.is_empty() {
                    None
                } else {
                    Some(dnd::AbilityGrant::Fixed(bonuses))
                }
            }
        })
        .collect()
}

fn parse_race_speed(speed: &Option<RawRaceSpeed>) -> dnd::Speed {
    match speed {
        Some(s) => dnd::Speed {
            walk: s.walk.unwrap_or(30),
            fly: s.fly.unwrap_or(0),
            swim: s.swim.unwrap_or(0),
            climb: s.climb.unwrap_or(0),
            burrow: s.burrow.unwrap_or(0),
        },
        None => dnd::Speed::default(),
    }
}

fn seed_races(conn: &DbConnection, data_dir: &Path) {
    let path = data_dir.join("races.json");
    if !path.exists() {
        info!("races.json not found, skipping");
        return;
    }

    info!("reading races");

    let content = std::fs::read_to_string(&path).expect("Failed to read races.json");
    let file: RaceFile = match serde_json::from_str(&content) {
        Ok(f) => f,
        Err(e) => {
            error!(%e, "failed to parse races.json");
            return;
        }
    };

    let mut total = 0;

    for race in file.race {
        let size = match race.size.first() {
            Some(&s) => convert::creature_size(s),
            None => continue,
        };

        let speed = convert::speed(parse_race_speed(&race.speed));
        let ability_grants: Vec<_> = parse_ability_grants(&race.ability)
            .into_iter()
            .map(convert::ability_grant)
            .collect();
        let lang_grants: Vec<_> = parse_language_grants(&race.language_proficiencies)
            .into_iter()
            .map(convert::language_grant)
            .collect();
        let description = entries_to_string(&race.entries);

        conn.reducers
            .seed_dnd_5_e_race(
                race.name,
                race.source,
                size,
                speed,
                ability_grants,
                lang_grants,
                description,
            )
            .ok();

        total += 1;
    }

    info!(count = total, "seeded races");
}

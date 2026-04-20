use crate::dnd5e::entry::Entry;
use dllm_core::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawStringList {
    One(String),
    Many(Vec<String>),
}

impl RawStringList {
    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::One(value) => vec![value],
            Self::Many(values) => values,
        }
    }
}

#[derive(Deserialize)]
pub struct RawTextEntry {
    pub entry: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawNumericValue {
    Number(u16),
    Special {
        #[serde(default)]
        _special: Option<String>,
    },
}

impl RawNumericValue {
    pub fn get(&self) -> Option<u16> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Special { .. } => None,
        }
    }
}

#[derive(Deserialize)]
pub struct ActionFile {
    pub action: Vec<RawAction>,
}

#[derive(Deserialize)]
pub struct RawAction {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub time: Vec<RawActionTime>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawActionTime {
    Structured { number: u16, unit: String },
    Text(String),
}

#[derive(Deserialize)]
pub struct LanguageFile {
    pub language: Vec<RawLanguage>,
}

#[derive(Deserialize)]
pub struct RawLanguage {
    pub name: String,
    pub source: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct SenseFile {
    pub sense: Vec<RawSense>,
}

#[derive(Deserialize)]
pub struct RawSense {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct SkillFile {
    pub skill: Vec<RawSkill>,
}

#[derive(Deserialize)]
pub struct RawSkill {
    pub name: String,
    pub source: String,
    pub ability: dnd::Ability,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct ObjectFile {
    pub object: Vec<RawObject>,
}

#[derive(Deserialize)]
pub struct RawObject {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub size: Vec<dnd::CreatureSize>,
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    #[serde(default)]
    pub ac: Option<RawNumericValue>,
    #[serde(default)]
    pub hp: Option<RawNumericValue>,
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(rename = "actionEntries", default)]
    pub action_entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct VehicleFile {
    pub vehicle: Vec<RawVehicle>,
}

#[derive(Deserialize)]
pub struct RawVehicle {
    pub name: String,
    pub source: String,
    #[serde(rename = "vehicleType")]
    pub vehicle_type: String,
    #[serde(default)]
    pub size: Option<RawStringList>,
    #[serde(default)]
    pub terrain: Vec<String>,
    #[serde(rename = "capCrew", default)]
    pub cap_crew: Option<u16>,
    #[serde(rename = "capPassenger", default)]
    pub cap_passenger: Option<u16>,
    #[serde(default)]
    pub pace: Option<RawVehiclePace>,
    #[serde(default)]
    pub ac: Option<RawVehicleArmor>,
    #[serde(default)]
    pub hp: Option<RawNumericValue>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawVehicleArmor {
    Number(u16),
    Special {
        #[serde(default)]
        _special: Option<String>,
    },
    Entries(Vec<RawVehicleArmorEntry>),
}

impl RawVehicleArmor {
    pub fn get(&self) -> Option<u16> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Special { .. } => None,
            Self::Entries(entries) => entries.iter().find_map(|entry| entry.ac),
        }
    }
}

#[derive(Deserialize)]
pub struct RawVehicleArmorEntry {
    #[serde(default)]
    pub ac: Option<u16>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawVehiclePace {
    Number(u16),
    Speeds {
        #[serde(default)]
        walk: Option<RawPaceValue>,
        #[serde(default)]
        fly: Option<RawPaceValue>,
        #[serde(default)]
        swim: Option<RawPaceValue>,
    },
}

impl RawVehiclePace {
    pub fn get(&self) -> Option<u16> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Speeds { walk, fly, swim } => walk
                .as_ref()
                .and_then(RawPaceValue::get)
                .or_else(|| fly.as_ref().and_then(RawPaceValue::get))
                .or_else(|| swim.as_ref().and_then(RawPaceValue::get)),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawPaceValue {
    Number(u16),
    Text(String),
}

impl RawPaceValue {
    pub fn get(&self) -> Option<u16> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Text(value) => value
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse()
                .ok(),
        }
    }
}

#[derive(Deserialize)]
pub struct DeityFile {
    pub deity: Vec<RawDeity>,
}

#[derive(Deserialize)]
pub struct RawDeity {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub pantheon: Option<String>,
    #[serde(default)]
    pub alignment: Vec<dnd::Alignment>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub province: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct RewardFile {
    pub reward: Vec<RawReward>,
}

#[derive(Deserialize)]
pub struct RawReward {
    pub name: String,
    pub source: String,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct TrapHazardFile {
    #[serde(default)]
    pub trap: Vec<RawTrapHazard>,
    #[serde(default)]
    pub hazard: Vec<RawTrapHazard>,
}

#[derive(Deserialize)]
pub struct RawTrapHazard {
    pub name: String,
    pub source: String,
    #[serde(rename = "trapHazType", default)]
    pub trap_hazard_type: Option<String>,
    #[serde(default)]
    pub trigger: Vec<Entry>,
    #[serde(default)]
    pub effect: Vec<Entry>,
    #[serde(default)]
    pub countermeasures: Vec<Entry>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct CharCreationOptionFile {
    pub charoption: Vec<RawCharCreationOption>,
}

#[derive(Deserialize)]
pub struct RawCharCreationOption {
    pub name: String,
    pub source: String,
    #[serde(rename = "optionType", default)]
    pub option_type: Vec<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct PsionicFile {
    pub psionic: Vec<RawPsionic>,
}

#[derive(Deserialize)]
pub struct RawPsionic {
    pub name: String,
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub order: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub focus: Option<String>,
    #[serde(default)]
    pub modes: Vec<RawPsionicMode>,
}

#[derive(Deserialize)]
pub struct RawPsionicMode {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub submodes: Vec<RawPsionicMode>,
}

#[derive(Deserialize)]
pub struct RecipeFile {
    pub recipe: Vec<RawRecipe>,
}

#[derive(Deserialize)]
pub struct RawRecipe {
    pub name: String,
    pub source: String,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    #[serde(rename = "dishTypes", default)]
    pub dish_types: Vec<String>,
    #[serde(default)]
    pub diet: Option<String>,
    #[serde(default)]
    pub serves: Option<RawRecipeServes>,
    #[serde(default)]
    pub ingredients: Vec<RawRecipeText>,
    #[serde(default)]
    pub instructions: Vec<RawRecipeText>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawRecipeText {
    Text(String),
    Structured {
        entry: String,
    },
    Section {
        #[serde(default)]
        name: Option<String>,
        entries: Vec<RawRecipeText>,
    },
}

impl RawRecipeText {
    pub fn into_text(self) -> String {
        match self {
            Self::Text(text) => text,
            Self::Structured { entry } => entry,
            Self::Section { name, entries } => {
                let body = entries
                    .into_iter()
                    .map(Self::into_text)
                    .filter(|text| !text.is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");
                match name {
                    Some(name) if !body.is_empty() => format!("{name}:\n{body}"),
                    Some(name) => name,
                    None => body,
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct RawRecipeServes {
    #[serde(default)]
    pub min: Option<u16>,
    #[serde(default)]
    pub max: Option<u16>,
    #[serde(default)]
    pub exact: Option<u16>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct CultsBoonsFile {
    #[serde(default)]
    pub cult: Vec<RawCultBoon>,
    #[serde(default)]
    pub boon: Vec<RawCultBoon>,
}

#[derive(Deserialize)]
pub struct RawCultBoon {
    pub name: String,
    pub source: String,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub goal: Option<RawTextEntry>,
    #[serde(default)]
    pub cultists: Option<RawTextEntry>,
    #[serde(rename = "signatureSpells", default)]
    pub signature_spells: Option<RawTextEntry>,
    #[serde(rename = "ability", default)]
    pub ability_text: Option<RawTextEntry>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct DeckFile {
    pub deck: Vec<RawDeck>,
}

#[derive(Deserialize)]
pub struct RawDeck {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub cards: Vec<RawDeckCardRef>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawDeckCardRef {
    Text(String),
    Structured {
        uid: String,
        #[serde(default)]
        count: Option<u16>,
        #[serde(default)]
        replacement: Option<bool>,
    },
}

impl RawDeckCardRef {
    pub fn into_summary(self) -> String {
        match self {
            Self::Text(text) => text,
            Self::Structured {
                uid,
                count,
                replacement,
            } => {
                let mut parts = vec![uid];
                if let Some(count) = count {
                    parts.push(format!("count={count}"));
                }
                if replacement.unwrap_or(false) {
                    parts.push("replacement".to_string());
                }
                parts.join(" | ")
            }
        }
    }
}

#[derive(Deserialize)]
pub struct VariantRuleFile {
    pub variantrule: Vec<RawVariantRule>,
}

#[derive(Deserialize)]
pub struct RawVariantRule {
    pub name: String,
    pub source: String,
    #[serde(rename = "ruleType", default)]
    pub rule_type: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

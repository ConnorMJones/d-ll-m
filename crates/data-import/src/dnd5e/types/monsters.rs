use dllm_core::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MonsterFile {
    pub monster: Vec<RawMonster>,
}

#[derive(Deserialize)]
pub struct RawMonster {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub size: Vec<dnd::CreatureSize>,
    #[serde(rename = "type", default)]
    pub creature_type: Option<CreatureTypeValue>,
    #[serde(default)]
    pub cr: Option<CrValue>,
    #[serde(default)]
    pub ac: Vec<AcValue>,
    #[serde(default)]
    pub hp: Option<HpValue>,
    #[serde(default)]
    pub speed: Option<SpeedValue>,
    #[serde(default, rename = "str")]
    pub str_score: u8,
    #[serde(default, rename = "dex")]
    pub dex_score: u8,
    #[serde(default, rename = "con")]
    pub con_score: u8,
    #[serde(default, rename = "int")]
    pub int_score: u8,
    #[serde(default, rename = "wis")]
    pub wis_score: u8,
    #[serde(default, rename = "cha")]
    pub cha_score: u8,
}

#[derive(Deserialize)]
pub struct CreatureTypeChoose {
    pub choose: Vec<dnd::CreatureType>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CreatureTypeInner {
    Fixed(dnd::CreatureType),
    Choose(CreatureTypeChoose),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CreatureTypeValue {
    Simple(dnd::CreatureType),
    Complex {
        #[serde(rename = "type")]
        creature_type: CreatureTypeInner,
    },
}

impl CreatureTypeValue {
    pub fn get(&self) -> Option<dnd::CreatureType> {
        match self {
            Self::Simple(t) => Some(*t),
            Self::Complex { creature_type } => match creature_type {
                CreatureTypeInner::Fixed(t) => Some(*t),
                CreatureTypeInner::Choose(c) => c.choose.first().copied(),
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CrValue {
    String(String),
    Number(u8),
    Complex { cr: String },
}

impl CrValue {
    pub fn get(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Complex { cr } => cr.clone(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AcValue {
    Number(u8),
    Complex {
        #[serde(default)]
        ac: Option<u8>,
        #[serde(default)]
        _special: Option<String>,
    },
}

impl AcValue {
    pub fn get(&self) -> u8 {
        match self {
            Self::Number(n) => *n,
            Self::Complex { ac, .. } => ac.unwrap_or(10),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct HpValue {
    #[serde(default)]
    pub average: u16,
    #[serde(default)]
    pub formula: Option<String>,
    #[serde(default)]
    pub _special: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SpeedNumber {
    Number(u16),
    Complex { number: u16 },
}

impl SpeedNumber {
    pub fn get(&self) -> u16 {
        match self {
            Self::Number(n) => *n,
            Self::Complex { number } => *number,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct SpeedValue {
    #[serde(default)]
    pub walk: Option<SpeedNumber>,
    #[serde(default)]
    pub fly: Option<SpeedNumber>,
    #[serde(default)]
    pub swim: Option<SpeedNumber>,
}

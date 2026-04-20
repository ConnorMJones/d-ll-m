use crate::dnd5e::entry::Entry;
use crate::dnd5e::types::{RawAbilityBlock, RawLanguageBlock};
use dllm_core::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RaceFile {
    pub race: Vec<RawRace>,
}

#[derive(Deserialize)]
pub struct RawRace {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub size: Vec<dnd::CreatureSize>,
    #[serde(default)]
    pub speed: Option<RawRaceSpeedValue>,
    #[serde(default)]
    pub ability: Vec<RawAbilityBlock>,
    #[serde(rename = "languageProficiencies", default)]
    pub language_proficiencies: Vec<RawLanguageBlock>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawRaceSpeedValue {
    Walk(u16),
    Full(RawRaceSpeed),
}

impl RawRaceSpeedValue {
    pub fn as_speed(&self) -> RawRaceSpeed {
        match self {
            Self::Walk(walk) => RawRaceSpeed {
                walk: Some(*walk),
                ..Default::default()
            },
            Self::Full(s) => s.clone(),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum SpeedEntry {
    Number(u16),
    EqualToWalk(bool),
}

impl SpeedEntry {
    pub fn resolve(&self, walk: u16) -> u16 {
        match self {
            Self::Number(n) => *n,
            Self::EqualToWalk(true) => walk,
            Self::EqualToWalk(false) => 0,
        }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct RawRaceSpeed {
    #[serde(default)]
    pub walk: Option<u16>,
    #[serde(default)]
    pub fly: Option<SpeedEntry>,
    #[serde(default)]
    pub swim: Option<SpeedEntry>,
    #[serde(default)]
    pub climb: Option<SpeedEntry>,
    #[serde(default)]
    pub burrow: Option<SpeedEntry>,
}

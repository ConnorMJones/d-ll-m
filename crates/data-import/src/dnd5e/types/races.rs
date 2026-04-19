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
    pub speed: Option<RawRaceSpeed>,
    #[serde(default)]
    pub ability: Vec<RawAbilityBlock>,
    #[serde(rename = "languageProficiencies", default)]
    pub language_proficiencies: Vec<RawLanguageBlock>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Default)]
pub struct RawRaceSpeed {
    #[serde(default)]
    pub walk: Option<u16>,
    #[serde(default)]
    pub fly: Option<u16>,
    #[serde(default)]
    pub swim: Option<u16>,
    #[serde(default)]
    pub climb: Option<u16>,
    #[serde(default)]
    pub burrow: Option<u16>,
}

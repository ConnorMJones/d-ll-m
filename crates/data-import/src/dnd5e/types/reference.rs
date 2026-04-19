use crate::dnd5e::entry::Entry;
use dllm_core::dnd5e as dnd;
use serde::Deserialize;

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

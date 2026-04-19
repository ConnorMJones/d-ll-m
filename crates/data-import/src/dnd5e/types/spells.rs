use crate::dnd5e::entry::Entry;
use dllm::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SpellFile {
    pub spell: Vec<RawSpell>,
}

#[derive(Deserialize)]
pub struct RawSpell {
    pub name: String,
    pub source: String,
    pub level: u8,
    pub school: dnd::SpellSchool,
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub duration: Vec<DurationEntry>,
    #[serde(default)]
    pub meta: Option<SpellMeta>,
    #[serde(rename = "savingThrow", default)]
    pub saving_throw: Vec<dnd::Ability>,
}

#[derive(Deserialize, Default)]
pub struct SpellMeta {
    #[serde(default)]
    pub ritual: bool,
}

#[derive(Deserialize)]
pub struct DurationEntry {
    #[serde(default)]
    pub concentration: bool,
}

use crate::dnd5e::entry::Entry;
use crate::dnd5e::types::{RawLanguageBlock, RawSkillBlock, RawToolBlock};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BackgroundFile {
    pub background: Vec<RawBackground>,
}

#[derive(Deserialize)]
pub struct RawBackground {
    pub name: String,
    pub source: String,
    #[serde(rename = "skillProficiencies", default)]
    pub skill_proficiencies: Vec<RawSkillBlock>,
    #[serde(rename = "toolProficiencies", default)]
    pub tool_proficiencies: Vec<RawToolBlock>,
    #[serde(rename = "languageProficiencies", default)]
    pub language_proficiencies: Vec<RawLanguageBlock>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

use crate::dnd5e::entry::Entry;
use dllm_core::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemFile {
    #[serde(default)]
    pub item: Vec<RawItem>,
    #[serde(default)]
    pub baseitem: Vec<RawItem>,
}

#[derive(Deserialize)]
pub struct RawItem {
    pub name: String,
    pub source: String,
    #[serde(rename = "type", default)]
    pub item_type: Option<RawItemType>,
    #[serde(default)]
    pub rarity: Option<dnd::ItemRarity>,
    #[serde(default)]
    pub weight: Option<f32>,
    #[serde(default)]
    pub value: Option<f32>,
    #[serde(default)]
    pub wondrous: bool,
    #[serde(rename = "reqAttune", default)]
    pub req_attune: Option<AttunementValue>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawItemType {
    Known(dnd::ItemType),
    Unknown(String),
}

impl RawItemType {
    pub fn get(&self) -> Option<dnd::ItemType> {
        match self {
            Self::Known(t) => Some(*t),
            Self::Unknown(s) => dnd::ItemType::from_code(s),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AttunementValue {
    Bool(bool),
    String(String),
}

impl AttunementValue {
    pub fn get(&self) -> Option<String> {
        match self {
            Self::Bool(true) => Some("requires attunement".to_string()),
            Self::Bool(false) => None,
            Self::String(s) => Some(format!("requires attunement {}", s)),
        }
    }
}

use crate::dnd5e::entry::Entry;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConditionFile {
    #[serde(default)]
    pub condition: Vec<RawCondition>,
    #[serde(default)]
    pub disease: Vec<RawCondition>,
}

#[derive(Deserialize)]
pub struct RawCondition {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

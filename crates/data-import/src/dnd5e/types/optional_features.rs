use crate::dnd5e::entry::Entry;
use dllm::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OptionalFeatureFile {
    pub optionalfeature: Vec<RawOptionalFeature>,
}

#[derive(Deserialize)]
pub struct RawOptionalFeature {
    pub name: String,
    pub source: String,
    #[serde(rename = "featureType", default)]
    pub feature_type: Vec<dnd::OptionalFeatureType>,
    #[serde(default)]
    pub prerequisite: Option<Vec<RawOptionalFeaturePrereq>>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Default)]
pub struct RawOptionalFeaturePrereq {
    #[serde(default)]
    pub level: Option<RawClassLevel>,
    #[serde(default)]
    pub pact: Option<dnd::PactBoon>,
    #[serde(default)]
    pub patron: Option<dnd::WarlockPatron>,
}

#[derive(Deserialize)]
pub struct RawClassLevel {
    pub level: u8,
    pub class: RawClassRef,
}

#[derive(Deserialize)]
pub struct RawClassRef {
    pub name: dnd::Class,
}

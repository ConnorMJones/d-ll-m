use dllm_core::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClassFile {
    #[serde(rename = "class")]
    pub classes: Vec<RawClass>,
    #[serde(default, rename = "subclass")]
    pub subclasses: Vec<RawSubclass>,
    #[serde(default, rename = "classFeature")]
    pub class_features: Vec<RawClassFeature>,
    #[serde(default, rename = "subclassFeature")]
    pub subclass_features: Vec<RawSubclassFeature>,
}

#[derive(Deserialize)]
pub struct RawClass {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub edition: Option<String>,
    pub hd: RawHitDie,
    #[serde(default, rename = "proficiency")]
    pub saving_throws: Vec<dnd::Ability>,
    #[serde(default, rename = "spellcastingAbility")]
    pub spellcasting_ability: Option<dnd::Ability>,
    #[serde(default, rename = "casterProgression")]
    pub caster_progression: Option<dnd::CasterProgression>,
    #[serde(default, rename = "preparedSpells")]
    pub prepared_spells_formula: Option<String>,
    #[serde(default, rename = "preparedSpellsProgression")]
    pub prepared_spells_progression: Vec<u8>,
    #[serde(default, rename = "cantripProgression")]
    pub cantrip_progression: Vec<u8>,
    #[serde(default, rename = "classFeatures")]
    pub class_features: Vec<RawClassFeatureRef>,
    #[serde(default, rename = "subclassTitle")]
    pub subclass_title: Option<String>,
}

#[derive(Deserialize)]
pub struct RawSubclass {
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub source: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "classSource")]
    pub class_source: String,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(default, rename = "spellcastingAbility")]
    pub spellcasting_ability: Option<dnd::Ability>,
    #[serde(default, rename = "casterProgression")]
    pub caster_progression: Option<dnd::CasterProgression>,
    #[serde(default, rename = "cantripProgression")]
    pub cantrip_progression: Vec<u8>,
    #[serde(default, rename = "subclassFeatures")]
    pub subclass_features: Vec<String>,
}

#[derive(Deserialize)]
pub struct RawClassFeature {
    pub name: String,
    pub source: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "classSource")]
    pub class_source: String,
    pub level: u8,
    #[serde(default)]
    pub entries: Vec<crate::dnd5e::entry::Entry>,
}

#[derive(Deserialize)]
pub struct RawSubclassFeature {
    pub name: String,
    pub source: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "classSource")]
    pub class_source: String,
    #[serde(rename = "subclassShortName")]
    pub subclass_short_name: String,
    #[serde(rename = "subclassSource")]
    pub subclass_source: String,
    pub level: u8,
    #[serde(default)]
    pub entries: Vec<crate::dnd5e::entry::Entry>,
}

#[derive(Deserialize)]
pub struct RawHitDie {
    pub faces: u8,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawClassFeatureRef {
    Simple(String),
    Detailed {
        #[serde(rename = "classFeature")]
        class_feature: String,
        #[serde(rename = "gainSubclassFeature", default)]
        _gain_subclass_feature: bool,
    },
}

impl RawClassFeatureRef {
    pub fn reference(&self) -> &str {
        match self {
            Self::Simple(reference) => reference,
            Self::Detailed { class_feature, .. } => class_feature,
        }
    }
}

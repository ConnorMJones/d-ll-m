use crate::dnd5e::entry::Entry;
use dllm::dnd5e as dnd;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FeatFile {
    pub feat: Vec<RawFeat>,
}

#[derive(Deserialize)]
pub struct RawFeat {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub category: Option<dnd::FeatCategory>,
    #[serde(default)]
    pub prerequisite: Option<Vec<RawFeatPrereq>>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Default)]
pub struct RawFeatPrereq {
    #[serde(default)]
    pub level: Option<u8>,
    #[serde(default)]
    pub race: Option<Vec<RawRaceRef>>,
    #[serde(default)]
    pub ability: Option<Vec<RawAbilityReq>>,
    #[serde(default)]
    pub spellcasting: Option<bool>,
}

#[derive(Deserialize)]
pub struct RawRaceRef {
    pub name: dnd::Race,
}

#[derive(Deserialize, Default)]
pub struct RawAbilityReq {
    #[serde(default, rename = "str")]
    pub strength: Option<u8>,
    #[serde(default, rename = "dex")]
    pub dexterity: Option<u8>,
    #[serde(default, rename = "con")]
    pub constitution: Option<u8>,
    #[serde(default, rename = "int")]
    pub intelligence: Option<u8>,
    #[serde(default, rename = "wis")]
    pub wisdom: Option<u8>,
    #[serde(default, rename = "cha")]
    pub charisma: Option<u8>,
}

impl RawAbilityReq {
    pub fn to_ability_scores(&self) -> Vec<dnd::AbilityScore> {
        [
            (dnd::Ability::Strength, self.strength),
            (dnd::Ability::Dexterity, self.dexterity),
            (dnd::Ability::Constitution, self.constitution),
            (dnd::Ability::Intelligence, self.intelligence),
            (dnd::Ability::Wisdom, self.wisdom),
            (dnd::Ability::Charisma, self.charisma),
        ]
        .into_iter()
        .filter_map(|(ability, min)| {
            min.map(|m| dnd::AbilityScore {
                ability,
                minimum: m,
            })
        })
        .collect()
    }
}

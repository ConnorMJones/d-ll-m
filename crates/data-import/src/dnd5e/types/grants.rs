use dllm_core::dnd5e as dnd;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
pub struct RawAbilityBlock {
    #[serde(default, rename = "str")]
    pub str_bonus: Option<i8>,
    #[serde(default, rename = "dex")]
    pub dex_bonus: Option<i8>,
    #[serde(default, rename = "con")]
    pub con_bonus: Option<i8>,
    #[serde(default, rename = "int")]
    pub int_bonus: Option<i8>,
    #[serde(default, rename = "wis")]
    pub wis_bonus: Option<i8>,
    #[serde(default, rename = "cha")]
    pub cha_bonus: Option<i8>,
    #[serde(default)]
    pub choose: Option<RawAbilityChoose>,
}

#[derive(Deserialize)]
pub struct RawAbilityChoose {
    #[serde(default)]
    pub count: u8,
    #[serde(default)]
    pub amount: Option<i8>,
}

#[derive(Deserialize, Default)]
pub struct RawSkillBlock {
    #[serde(default)]
    pub acrobatics: Option<bool>,
    #[serde(rename = "animal handling", default)]
    pub animal_handling: Option<bool>,
    #[serde(default)]
    pub arcana: Option<bool>,
    #[serde(default)]
    pub athletics: Option<bool>,
    #[serde(default)]
    pub deception: Option<bool>,
    #[serde(default)]
    pub history: Option<bool>,
    #[serde(default)]
    pub insight: Option<bool>,
    #[serde(default)]
    pub intimidation: Option<bool>,
    #[serde(default)]
    pub investigation: Option<bool>,
    #[serde(default)]
    pub medicine: Option<bool>,
    #[serde(default)]
    pub nature: Option<bool>,
    #[serde(default)]
    pub perception: Option<bool>,
    #[serde(default)]
    pub performance: Option<bool>,
    #[serde(default)]
    pub persuasion: Option<bool>,
    #[serde(default)]
    pub religion: Option<bool>,
    #[serde(rename = "sleight of hand", default)]
    pub sleight_of_hand: Option<bool>,
    #[serde(default)]
    pub stealth: Option<bool>,
    #[serde(default)]
    pub survival: Option<bool>,
    #[serde(default)]
    pub any: Option<u8>,
    #[serde(default)]
    pub choose: Option<RawSkillChoose>,
}

impl RawSkillBlock {
    pub fn to_fixed_skills(&self) -> Vec<dnd::Skill> {
        [
            (dnd::Skill::Acrobatics, self.acrobatics),
            (dnd::Skill::AnimalHandling, self.animal_handling),
            (dnd::Skill::Arcana, self.arcana),
            (dnd::Skill::Athletics, self.athletics),
            (dnd::Skill::Deception, self.deception),
            (dnd::Skill::History, self.history),
            (dnd::Skill::Insight, self.insight),
            (dnd::Skill::Intimidation, self.intimidation),
            (dnd::Skill::Investigation, self.investigation),
            (dnd::Skill::Medicine, self.medicine),
            (dnd::Skill::Nature, self.nature),
            (dnd::Skill::Perception, self.perception),
            (dnd::Skill::Performance, self.performance),
            (dnd::Skill::Persuasion, self.persuasion),
            (dnd::Skill::Religion, self.religion),
            (dnd::Skill::SleightOfHand, self.sleight_of_hand),
            (dnd::Skill::Stealth, self.stealth),
            (dnd::Skill::Survival, self.survival),
        ]
        .into_iter()
        .filter(|(_, v)| *v == Some(true))
        .map(|(skill, _)| skill)
        .collect()
    }
}

#[derive(Deserialize)]
pub struct RawSkillChoose {
    #[serde(default)]
    pub from: Vec<dnd::Skill>,
    #[serde(default)]
    pub count: Option<u8>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum BoolOrInt {
    Bool(bool),
    Int(u8),
}

impl BoolOrInt {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Int(n) => *n > 0,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct RawToolBlock {
    #[serde(default)]
    pub any: Option<u8>,
    #[serde(default)]
    pub choose: Option<RawToolChoose>,
    #[serde(flatten)]
    pub tools: HashMap<String, BoolOrInt>,
}

#[derive(Deserialize)]
pub struct RawToolChoose {
    #[serde(default)]
    pub from: Vec<String>,
    #[serde(default)]
    pub count: Option<u8>,
}

#[derive(Deserialize, Default)]
pub struct RawLanguageBlock {
    #[serde(default)]
    pub common: Option<bool>,
    #[serde(default)]
    pub dwarvish: Option<bool>,
    #[serde(default)]
    pub elvish: Option<bool>,
    #[serde(default)]
    pub giant: Option<bool>,
    #[serde(default)]
    pub gnomish: Option<bool>,
    #[serde(default)]
    pub goblin: Option<bool>,
    #[serde(default)]
    pub halfling: Option<bool>,
    #[serde(default)]
    pub orc: Option<bool>,
    #[serde(default)]
    pub abyssal: Option<bool>,
    #[serde(default)]
    pub celestial: Option<bool>,
    #[serde(rename = "deep speech", default)]
    pub deep_speech: Option<bool>,
    #[serde(default)]
    pub draconic: Option<bool>,
    #[serde(default)]
    pub infernal: Option<bool>,
    #[serde(default)]
    pub primordial: Option<bool>,
    #[serde(default)]
    pub sylvan: Option<bool>,
    #[serde(default)]
    pub undercommon: Option<bool>,
    #[serde(default)]
    pub aquan: Option<bool>,
    #[serde(default)]
    pub auran: Option<bool>,
    #[serde(default)]
    pub ignan: Option<bool>,
    #[serde(default)]
    pub terran: Option<bool>,
    #[serde(default)]
    pub druidic: Option<bool>,
    #[serde(rename = "thieves' cant", default)]
    pub thieves_cant: Option<bool>,
    #[serde(default)]
    pub gith: Option<bool>,
    #[serde(rename = "anyStandard", default)]
    pub any_standard: Option<u8>,
    #[serde(rename = "anyExotic", default)]
    pub any_exotic: Option<u8>,
    #[serde(default)]
    pub any: Option<u8>,
    #[serde(default)]
    pub choose: Option<RawLanguageChoose>,
}

impl RawLanguageBlock {
    pub fn to_fixed_languages(&self) -> Vec<dnd::Language> {
        [
            (dnd::Language::Common, self.common),
            (dnd::Language::Dwarvish, self.dwarvish),
            (dnd::Language::Elvish, self.elvish),
            (dnd::Language::Giant, self.giant),
            (dnd::Language::Gnomish, self.gnomish),
            (dnd::Language::Goblin, self.goblin),
            (dnd::Language::Halfling, self.halfling),
            (dnd::Language::Orc, self.orc),
            (dnd::Language::Abyssal, self.abyssal),
            (dnd::Language::Celestial, self.celestial),
            (dnd::Language::DeepSpeech, self.deep_speech),
            (dnd::Language::Draconic, self.draconic),
            (dnd::Language::Infernal, self.infernal),
            (dnd::Language::Primordial, self.primordial),
            (dnd::Language::Sylvan, self.sylvan),
            (dnd::Language::Undercommon, self.undercommon),
            (dnd::Language::Aquan, self.aquan),
            (dnd::Language::Auran, self.auran),
            (dnd::Language::Ignan, self.ignan),
            (dnd::Language::Terran, self.terran),
            (dnd::Language::Druidic, self.druidic),
            (dnd::Language::ThievesCant, self.thieves_cant),
            (dnd::Language::Gith, self.gith),
        ]
        .into_iter()
        .filter(|(_, v)| *v == Some(true))
        .map(|(lang, _)| lang)
        .collect()
    }
}

#[derive(Deserialize)]
pub struct RawLanguageChoose {
    #[serde(default)]
    pub from: Vec<dnd::Language>,
    #[serde(default)]
    pub count: Option<u8>,
}

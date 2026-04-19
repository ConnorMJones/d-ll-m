use dllm_core::dnd5e::{
    Ability, AbilityGrant, CasterProgression, CreatureSize, CreatureType, FeatCategory, FeatPrereq,
    ItemRarity, ItemType, LanguageGrant, OptionalFeaturePrereq, OptionalFeatureType, SkillGrant,
    Speed, SpellSchool, ToolGrant,
};

#[spacetimedb::table(accessor = user, public)]
pub struct User {
    #[primary_key]
    pub identity: spacetimedb::Identity,
    pub name: Option<String>,
    pub online: bool,
}

#[spacetimedb::table(accessor = message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub sender: spacetimedb::Identity,
    pub text: String,
    pub sent: spacetimedb::Timestamp,
}

#[spacetimedb::table(accessor = dnd5e_spell, public)]
pub struct Dnd5eSpell {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub level: u8,
    pub school: SpellSchool,
    pub ritual: bool,
    pub concentration: bool,
    pub description: String,
    #[index(btree)]
    pub saving_throw: Option<Ability>,
}

#[spacetimedb::table(accessor = dnd5e_monster, public)]
pub struct Dnd5eMonster {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub size: CreatureSize,
    pub creature_type: CreatureType,
    #[index(btree)]
    pub cr: String,
    pub ac: u8,
    pub hp_average: u16,
    pub hp_formula: String,
    pub speed_walk: u16,
    pub speed_fly: u16,
    pub speed_swim: u16,
    pub str_score: u8,
    pub dex_score: u8,
    pub con_score: u8,
    pub int_score: u8,
    pub wis_score: u8,
    pub cha_score: u8,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_item, public)]
pub struct Dnd5eItem {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    #[index(btree)]
    pub item_type: ItemType,
    #[index(btree)]
    pub rarity: ItemRarity,
    pub weight: Option<f32>,
    pub value_cp: Option<u32>,
    pub wondrous: bool,
    pub attunement: Option<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_feat, public)]
pub struct Dnd5eFeat {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    #[index(btree)]
    pub category: Option<FeatCategory>,
    pub prerequisite: Option<FeatPrereq>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_condition, public)]
pub struct Dnd5eCondition {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_background, public)]
pub struct Dnd5eBackground {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub skill_proficiencies: Vec<SkillGrant>,
    pub tool_proficiencies: Vec<ToolGrant>,
    pub language_proficiencies: Vec<LanguageGrant>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_race, public)]
pub struct Dnd5eRace {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    #[index(btree)]
    pub size: CreatureSize,
    pub speed: Speed,
    pub ability_bonuses: Vec<AbilityGrant>,
    pub language_proficiencies: Vec<LanguageGrant>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_optional_feature, public)]
pub struct Dnd5eOptionalFeature {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub feature_types: Vec<OptionalFeatureType>,
    pub prerequisite: Option<OptionalFeaturePrereq>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_action, public)]
pub struct Dnd5eAction {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub time: String,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_language, public)]
pub struct Dnd5eLanguage {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub kind: Option<String>,
    pub script: Option<String>,
    pub origin: Option<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_sense, public)]
pub struct Dnd5eSense {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_skill, public)]
pub struct Dnd5eSkill {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub ability: Ability,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_class, public)]
pub struct Dnd5eClass {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub edition: Option<String>,
    pub hit_die: u8,
    pub saving_throws: Vec<Ability>,
    pub spellcasting_ability: Option<Ability>,
    pub caster_progression: Option<CasterProgression>,
    pub prepared_spells_formula: Option<String>,
    pub prepared_spells_progression: Vec<u8>,
    pub cantrip_progression: Vec<u8>,
    pub class_features: Vec<String>,
    pub subclass_title: Option<String>,
}

#[spacetimedb::table(accessor = dnd5e_subclass, public)]
pub struct Dnd5eSubclass {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub short_name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub edition: Option<String>,
    pub spellcasting_ability: Option<Ability>,
    pub caster_progression: Option<CasterProgression>,
    pub cantrip_progression: Vec<u8>,
    pub subclass_features: Vec<String>,
}

#[spacetimedb::table(accessor = dnd5e_class_feature, public)]
pub struct Dnd5eClassFeature {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub level: u8,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_subclass_feature, public)]
pub struct Dnd5eSubclassFeature {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub class_name: String,
    pub class_source: String,
    pub subclass_short_name: String,
    pub subclass_source: String,
    pub level: u8,
    pub description: String,
}

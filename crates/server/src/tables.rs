use dllm_core::dnd5e::{
    Ability, AbilityGrant, Alignment, CasterProgression, CreatureSize, CreatureType, FeatCategory,
    FeatPrereq, ItemRarity, ItemType, LanguageGrant, OptionalFeaturePrereq, OptionalFeatureType,
    SkillGrant, Speed, SpellSchool, ToolGrant,
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

#[spacetimedb::table(accessor = dnd5e_object, public)]
pub struct Dnd5eObject {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub size: Vec<CreatureSize>,
    pub object_type: Option<String>,
    pub ac: Option<u16>,
    pub hp: Option<u16>,
    pub description: String,
    pub action_description: String,
}

#[spacetimedb::table(accessor = dnd5e_vehicle, public)]
pub struct Dnd5eVehicle {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub vehicle_type: String,
    pub size: Vec<String>,
    pub terrain: Vec<String>,
    pub crew_capacity: Option<u16>,
    pub passenger_capacity: Option<u16>,
    pub pace: Option<u16>,
    pub ac: Option<u16>,
    pub hp: Option<u16>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_deity, public)]
pub struct Dnd5eDeity {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub pantheon: Option<String>,
    pub alignment: Vec<Alignment>,
    pub category: Option<String>,
    pub domains: Vec<String>,
    pub province: Option<String>,
    pub title: Option<String>,
    pub symbol: Option<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_reward, public)]
pub struct Dnd5eReward {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub reward_type: Option<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_trap_hazard, public)]
pub struct Dnd5eTrapHazard {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub kind: String,
    pub trap_hazard_type: Option<String>,
    pub trigger: String,
    pub effect: String,
    pub countermeasures: String,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_char_creation_option, public)]
pub struct Dnd5eCharCreationOption {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub option_types: Vec<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_psionic, public)]
pub struct Dnd5ePsionic {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub kind: String,
    pub order_name: Option<String>,
    pub focus: Option<String>,
    pub description: String,
    pub modes: String,
}

#[spacetimedb::table(accessor = dnd5e_recipe, public)]
pub struct Dnd5eRecipe {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub recipe_type: Option<String>,
    pub dish_types: Vec<String>,
    pub diet: Option<String>,
    pub serves: Option<String>,
    pub ingredients: String,
    pub instructions: String,
}

#[spacetimedb::table(accessor = dnd5e_cult_boon, public)]
pub struct Dnd5eCultBoon {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub kind: String,
    pub subtype: Option<String>,
    pub goal: Option<String>,
    pub cultists: Option<String>,
    pub signature_spells: Option<String>,
    pub ability_text: Option<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_deck, public)]
pub struct Dnd5eDeck {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub cards: Vec<String>,
    pub description: String,
}

#[spacetimedb::table(accessor = dnd5e_variant_rule, public)]
pub struct Dnd5eVariantRule {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub source: String,
    pub rule_type: Option<String>,
    pub description: String,
}

use dllm::dnd5e::{
    Ability, AbilityGrant, CreatureSize, CreatureType, FeatCategory, FeatPrereq, ItemRarity,
    ItemType, LanguageGrant, OptionalFeaturePrereq, OptionalFeatureType, SkillGrant, Speed,
    SpellSchool, ToolGrant,
};
use spacetimedb::{ReducerContext, Table};

#[spacetimedb::table(accessor = user, public)]
pub struct User {
    #[primary_key]
    identity: spacetimedb::Identity,
    name: Option<String>,
    online: bool,
}

#[spacetimedb::table(accessor = message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    id: u64,
    sender: spacetimedb::Identity,
    text: String,
    sent: spacetimedb::Timestamp,
}

#[spacetimedb::table(accessor = dnd5e_spell, public)]
pub struct Dnd5eSpell {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    level: u8,
    school: SpellSchool,
    ritual: bool,
    concentration: bool,
    description: String,
    #[index(btree)]
    saving_throw: Option<Ability>,
}

#[spacetimedb::table(accessor = dnd5e_monster, public)]
pub struct Dnd5eMonster {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    size: CreatureSize,
    creature_type: CreatureType,
    #[index(btree)]
    cr: String,
    ac: u8,
    hp_average: u16,
    hp_formula: String,
    speed_walk: u16,
    speed_fly: u16,
    speed_swim: u16,
    str_score: u8,
    dex_score: u8,
    con_score: u8,
    int_score: u8,
    wis_score: u8,
    cha_score: u8,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_item, public)]
pub struct Dnd5eItem {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    #[index(btree)]
    item_type: ItemType,
    #[index(btree)]
    rarity: ItemRarity,
    weight: Option<f32>,
    value_cp: Option<u32>,
    wondrous: bool,
    attunement: Option<String>,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_feat, public)]
pub struct Dnd5eFeat {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    #[index(btree)]
    category: Option<FeatCategory>,
    prerequisite: Option<FeatPrereq>,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_condition, public)]
pub struct Dnd5eCondition {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_background, public)]
pub struct Dnd5eBackground {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    skill_proficiencies: Vec<SkillGrant>,
    tool_proficiencies: Vec<ToolGrant>,
    language_proficiencies: Vec<LanguageGrant>,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_race, public)]
pub struct Dnd5eRace {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    #[index(btree)]
    size: CreatureSize,
    speed: Speed,
    ability_bonuses: Vec<AbilityGrant>,
    language_proficiencies: Vec<LanguageGrant>,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_optional_feature, public)]
pub struct Dnd5eOptionalFeature {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    feature_types: Vec<OptionalFeatureType>,
    prerequisite: Option<OptionalFeaturePrereq>,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_action, public)]
pub struct Dnd5eAction {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    time: String,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_language, public)]
pub struct Dnd5eLanguage {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    kind: Option<String>,
    script: Option<String>,
    origin: Option<String>,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_sense, public)]
pub struct Dnd5eSense {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    description: String,
}

#[spacetimedb::table(accessor = dnd5e_skill, public)]
pub struct Dnd5eSkill {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    source: String,
    ability: Ability,
    description: String,
}

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        ctx.db.user().insert(User {
            identity: ctx.sender(),
            name: None,
            online: true,
        });
    }
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
    }
}

#[spacetimedb::reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User {
            name: Some(name),
            ..user
        });
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}

#[spacetimedb::reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    if text.is_empty() {
        return Err("Message cannot be empty".to_string());
    }
    ctx.db.message().insert(Message {
        id: 0,
        sender: ctx.sender(),
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_spell(
    ctx: &ReducerContext,
    name: String,
    source: String,
    level: u8,
    school: SpellSchool,
    ritual: bool,
    concentration: bool,
    description: String,
    saving_throw: Option<Ability>,
) -> Result<(), String> {
    ctx.db.dnd5e_spell().insert(Dnd5eSpell {
        id: 0,
        name,
        source,
        level,
        school,
        ritual,
        concentration,
        description,
        saving_throw,
    });
    Ok(())
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_monster(
    ctx: &ReducerContext,
    name: String,
    source: String,
    size: CreatureSize,
    creature_type: CreatureType,
    cr: String,
    ac: u8,
    hp_average: u16,
    hp_formula: String,
    speed_walk: u16,
    speed_fly: u16,
    speed_swim: u16,
    str_score: u8,
    dex_score: u8,
    con_score: u8,
    int_score: u8,
    wis_score: u8,
    cha_score: u8,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_monster().insert(Dnd5eMonster {
        id: 0,
        name,
        source,
        size,
        creature_type,
        cr,
        ac,
        hp_average,
        hp_formula,
        speed_walk,
        speed_fly,
        speed_swim,
        str_score,
        dex_score,
        con_score,
        int_score,
        wis_score,
        cha_score,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_item(
    ctx: &ReducerContext,
    name: String,
    source: String,
    item_type: ItemType,
    rarity: ItemRarity,
    weight: Option<f32>,
    value_cp: Option<u32>,
    wondrous: bool,
    attunement: Option<String>,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_item().insert(Dnd5eItem {
        id: 0,
        name,
        source,
        item_type,
        rarity,
        weight,
        value_cp,
        wondrous,
        attunement,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_feat(
    ctx: &ReducerContext,
    name: String,
    source: String,
    category: Option<FeatCategory>,
    prerequisite: Option<FeatPrereq>,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_feat().insert(Dnd5eFeat {
        id: 0,
        name,
        source,
        category,
        prerequisite,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_condition(
    ctx: &ReducerContext,
    name: String,
    source: String,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_condition().insert(Dnd5eCondition {
        id: 0,
        name,
        source,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_background(
    ctx: &ReducerContext,
    name: String,
    source: String,
    skill_proficiencies: Vec<SkillGrant>,
    tool_proficiencies: Vec<ToolGrant>,
    language_proficiencies: Vec<LanguageGrant>,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_background().insert(Dnd5eBackground {
        id: 0,
        name,
        source,
        skill_proficiencies,
        tool_proficiencies,
        language_proficiencies,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_race(
    ctx: &ReducerContext,
    name: String,
    source: String,
    size: CreatureSize,
    speed: Speed,
    ability_bonuses: Vec<AbilityGrant>,
    language_proficiencies: Vec<LanguageGrant>,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_race().insert(Dnd5eRace {
        id: 0,
        name,
        source,
        size,
        speed,
        ability_bonuses,
        language_proficiencies,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_optional_feature(
    ctx: &ReducerContext,
    name: String,
    source: String,
    feature_types: Vec<OptionalFeatureType>,
    prerequisite: Option<OptionalFeaturePrereq>,
    description: String,
) -> Result<(), String> {
    ctx.db
        .dnd5e_optional_feature()
        .insert(Dnd5eOptionalFeature {
            id: 0,
            name,
            source,
            feature_types,
            prerequisite,
            description,
        });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_action(
    ctx: &ReducerContext,
    name: String,
    source: String,
    time: String,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_action().insert(Dnd5eAction {
        id: 0,
        name,
        source,
        time,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_language(
    ctx: &ReducerContext,
    name: String,
    source: String,
    kind: Option<String>,
    script: Option<String>,
    origin: Option<String>,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_language().insert(Dnd5eLanguage {
        id: 0,
        name,
        source,
        kind,
        script,
        origin,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_sense(
    ctx: &ReducerContext,
    name: String,
    source: String,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_sense().insert(Dnd5eSense {
        id: 0,
        name,
        source,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_skill(
    ctx: &ReducerContext,
    name: String,
    source: String,
    ability: Ability,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_skill().insert(Dnd5eSkill {
        id: 0,
        name,
        source,
        ability,
        description,
    });
    Ok(())
}

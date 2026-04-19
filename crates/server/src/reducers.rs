use crate::tables::*;
use dllm_core::dnd5e::{
    Ability, AbilityGrant, CasterProgression, CreatureSize, CreatureType, FeatCategory, FeatPrereq,
    ItemRarity, ItemType, LanguageGrant, OptionalFeaturePrereq, OptionalFeatureType, SkillGrant,
    Speed, SpellSchool, ToolGrant,
};
use spacetimedb::{ReducerContext, Table};

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

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_class(
    ctx: &ReducerContext,
    name: String,
    source: String,
    edition: Option<String>,
    hit_die: u8,
    saving_throws: Vec<Ability>,
    spellcasting_ability: Option<Ability>,
    caster_progression: Option<CasterProgression>,
    prepared_spells_formula: Option<String>,
    prepared_spells_progression: Vec<u8>,
    cantrip_progression: Vec<u8>,
    class_features: Vec<String>,
    subclass_title: Option<String>,
) -> Result<(), String> {
    ctx.db.dnd5e_class().insert(Dnd5eClass {
        id: 0,
        name,
        source,
        edition,
        hit_die,
        saving_throws,
        spellcasting_ability,
        caster_progression,
        prepared_spells_formula,
        prepared_spells_progression,
        cantrip_progression,
        class_features,
        subclass_title,
    });
    Ok(())
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_subclass(
    ctx: &ReducerContext,
    name: String,
    short_name: String,
    source: String,
    class_name: String,
    class_source: String,
    edition: Option<String>,
    spellcasting_ability: Option<Ability>,
    caster_progression: Option<CasterProgression>,
    cantrip_progression: Vec<u8>,
    subclass_features: Vec<String>,
) -> Result<(), String> {
    ctx.db.dnd5e_subclass().insert(Dnd5eSubclass {
        id: 0,
        name,
        short_name,
        source,
        class_name,
        class_source,
        edition,
        spellcasting_ability,
        caster_progression,
        cantrip_progression,
        subclass_features,
    });
    Ok(())
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_class_feature(
    ctx: &ReducerContext,
    name: String,
    source: String,
    class_name: String,
    class_source: String,
    level: u8,
    description: String,
) -> Result<(), String> {
    ctx.db.dnd5e_class_feature().insert(Dnd5eClassFeature {
        id: 0,
        name,
        source,
        class_name,
        class_source,
        level,
        description,
    });
    Ok(())
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_subclass_feature(
    ctx: &ReducerContext,
    name: String,
    source: String,
    class_name: String,
    class_source: String,
    subclass_short_name: String,
    subclass_source: String,
    level: u8,
    description: String,
) -> Result<(), String> {
    ctx.db
        .dnd5e_subclass_feature()
        .insert(Dnd5eSubclassFeature {
            id: 0,
            name,
            source,
            class_name,
            class_source,
            subclass_short_name,
            subclass_source,
            level,
            description,
        });
    Ok(())
}

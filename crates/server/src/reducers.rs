use crate::tables::*;
use dllm_core::dnd5e::{
    Ability, AbilityGrant, Alignment, CasterProgression, CreatureSize, CreatureType, FeatCategory,
    FeatPrereq, ItemRarity, ItemType, LanguageGrant, OptionalFeaturePrereq, OptionalFeatureType,
    SkillGrant, Speed, SpellSchool, ToolGrant, class_feature_key, source_name_key,
    subclass_feature_key, subclass_key,
};
use spacetimedb::{ReducerContext, Table, TryInsertError};

macro_rules! insert_seed_row {
    ($table:expr, $row:expr) => {
        match $table.try_insert($row) {
            Ok(_) | Err(TryInsertError::UniqueConstraintViolation(_)) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    };
}

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    let display_name = ensure_profile(ctx);

    if let Some(user) = ctx.db.user().identity().find(ctx.sender()) {
        ctx.db.user().identity().update(User {
            online: true,
            name: display_name,
            ..user
        });
    } else {
        ctx.db.user().insert(User {
            identity: ctx.sender(),
            name: display_name,
            online: true,
        });
    }
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    touch_profile_last_seen(ctx);

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
    upsert_profile_name(ctx, Some(name.clone()));
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

fn ensure_profile(ctx: &ReducerContext) -> Option<String> {
    if let Some(profile) = ctx.db.profile().identity().find(ctx.sender()) {
        let display_name = profile.display_name.clone();
        ctx.db.profile().identity().update(Profile {
            last_seen_at: ctx.timestamp,
            ..profile
        });
        display_name
    } else {
        ctx.db.profile().insert(Profile {
            identity: ctx.sender(),
            display_name: None,
            created_at: ctx.timestamp,
            last_seen_at: ctx.timestamp,
        });
        None
    }
}

fn touch_profile_last_seen(ctx: &ReducerContext) {
    if let Some(profile) = ctx.db.profile().identity().find(ctx.sender()) {
        ctx.db.profile().identity().update(Profile {
            last_seen_at: ctx.timestamp,
            ..profile
        });
    }
}

fn upsert_profile_name(ctx: &ReducerContext, display_name: Option<String>) {
    if let Some(profile) = ctx.db.profile().identity().find(ctx.sender()) {
        ctx.db.profile().identity().update(Profile {
            display_name,
            last_seen_at: ctx.timestamp,
            ..profile
        });
    } else {
        ctx.db.profile().insert(Profile {
            identity: ctx.sender(),
            display_name,
            created_at: ctx.timestamp,
            last_seen_at: ctx.timestamp,
        });
    }
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
    insert_seed_row!(
        ctx.db.dnd5e_spell(),
        Dnd5eSpell {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            level,
            school,
            ritual,
            concentration,
            description,
            saving_throw,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_monster(),
        Dnd5eMonster {
            id: 0,
            key: source_name_key(&source, &name),
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
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_item(),
        Dnd5eItem {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            item_type,
            rarity,
            weight,
            value_cp,
            wondrous,
            attunement,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_feat(),
        Dnd5eFeat {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            category,
            prerequisite,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_condition(
    ctx: &ReducerContext,
    name: String,
    source: String,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_condition(),
        Dnd5eCondition {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_background(),
        Dnd5eBackground {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            skill_proficiencies,
            tool_proficiencies,
            language_proficiencies,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_race(),
        Dnd5eRace {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            size,
            speed,
            ability_bonuses,
            language_proficiencies,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_optional_feature(),
        Dnd5eOptionalFeature {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            feature_types,
            prerequisite,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_action(
    ctx: &ReducerContext,
    name: String,
    source: String,
    time: String,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_action(),
        Dnd5eAction {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            time,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_language(),
        Dnd5eLanguage {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            kind,
            script,
            origin,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_sense(
    ctx: &ReducerContext,
    name: String,
    source: String,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_sense(),
        Dnd5eSense {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_skill(
    ctx: &ReducerContext,
    name: String,
    source: String,
    ability: Ability,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_skill(),
        Dnd5eSkill {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            ability,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_class(),
        Dnd5eClass {
            id: 0,
            key: source_name_key(&source, &name),
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
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_subclass(),
        Dnd5eSubclass {
            id: 0,
            key: subclass_key(&source, &class_source, &class_name, &short_name),
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
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_class_feature(),
        Dnd5eClassFeature {
            id: 0,
            key: class_feature_key(&source, &class_source, &class_name, level, &name),
            name,
            source,
            class_name,
            class_source,
            level,
            description,
        }
    )
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
    insert_seed_row!(
        ctx.db.dnd5e_subclass_feature(),
        Dnd5eSubclassFeature {
            id: 0,
            key: subclass_feature_key(
                &source,
                &class_source,
                &class_name,
                &subclass_source,
                &subclass_short_name,
                level,
                &name,
            ),
            name,
            source,
            class_name,
            class_source,
            subclass_short_name,
            subclass_source,
            level,
            description,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_object(
    ctx: &ReducerContext,
    name: String,
    source: String,
    size: Vec<CreatureSize>,
    object_type: Option<String>,
    ac: Option<u16>,
    hp: Option<u16>,
    description: String,
    action_description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_object(),
        Dnd5eObject {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            size,
            object_type,
            ac,
            hp,
            description,
            action_description,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_vehicle(
    ctx: &ReducerContext,
    name: String,
    source: String,
    vehicle_type: String,
    size: Vec<String>,
    terrain: Vec<String>,
    crew_capacity: Option<u16>,
    passenger_capacity: Option<u16>,
    pace: Option<u16>,
    ac: Option<u16>,
    hp: Option<u16>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_vehicle(),
        Dnd5eVehicle {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            vehicle_type,
            size,
            terrain,
            crew_capacity,
            passenger_capacity,
            pace,
            ac,
            hp,
            description,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_deity(
    ctx: &ReducerContext,
    name: String,
    source: String,
    pantheon: Option<String>,
    alignment: Vec<Alignment>,
    category: Option<String>,
    domains: Vec<String>,
    province: Option<String>,
    title: Option<String>,
    symbol: Option<String>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_deity(),
        Dnd5eDeity {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            pantheon,
            alignment,
            category,
            domains,
            province,
            title,
            symbol,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_reward(
    ctx: &ReducerContext,
    name: String,
    source: String,
    reward_type: Option<String>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_reward(),
        Dnd5eReward {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            reward_type,
            description,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_trap_hazard(
    ctx: &ReducerContext,
    name: String,
    source: String,
    kind: String,
    trap_hazard_type: Option<String>,
    trigger: String,
    effect: String,
    countermeasures: String,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_trap_hazard(),
        Dnd5eTrapHazard {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            kind,
            trap_hazard_type,
            trigger,
            effect,
            countermeasures,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_char_creation_option(
    ctx: &ReducerContext,
    name: String,
    source: String,
    option_types: Vec<String>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_char_creation_option(),
        Dnd5eCharCreationOption {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            option_types,
            description,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_psionic(
    ctx: &ReducerContext,
    name: String,
    source: String,
    kind: String,
    order_name: Option<String>,
    focus: Option<String>,
    description: String,
    modes: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_psionic(),
        Dnd5ePsionic {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            kind,
            order_name,
            focus,
            description,
            modes,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_recipe(
    ctx: &ReducerContext,
    name: String,
    source: String,
    recipe_type: Option<String>,
    dish_types: Vec<String>,
    diet: Option<String>,
    serves: Option<String>,
    ingredients: String,
    instructions: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_recipe(),
        Dnd5eRecipe {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            recipe_type,
            dish_types,
            diet,
            serves,
            ingredients,
            instructions,
        }
    )
}

#[spacetimedb::reducer]
#[allow(clippy::too_many_arguments)]
pub fn seed_dnd5e_cult_boon(
    ctx: &ReducerContext,
    name: String,
    source: String,
    kind: String,
    subtype: Option<String>,
    goal: Option<String>,
    cultists: Option<String>,
    signature_spells: Option<String>,
    ability_text: Option<String>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_cult_boon(),
        Dnd5eCultBoon {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            kind,
            subtype,
            goal,
            cultists,
            signature_spells,
            ability_text,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_deck(
    ctx: &ReducerContext,
    name: String,
    source: String,
    cards: Vec<String>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_deck(),
        Dnd5eDeck {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            cards,
            description,
        }
    )
}

#[spacetimedb::reducer]
pub fn seed_dnd5e_variant_rule(
    ctx: &ReducerContext,
    name: String,
    source: String,
    rule_type: Option<String>,
    description: String,
) -> Result<(), String> {
    insert_seed_row!(
        ctx.db.dnd5e_variant_rule(),
        Dnd5eVariantRule {
            id: 0,
            key: source_name_key(&source, &name),
            name,
            source,
            rule_type,
            description,
        }
    )
}

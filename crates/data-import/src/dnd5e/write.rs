use crate::dnd5e::normalize::{
    ImportAction, ImportBackground, ImportCharCreationOption, ImportClass, ImportClassFeature,
    ImportCondition, ImportCultBoon, ImportDeck, ImportDeity, ImportFeat, ImportItem,
    ImportLanguage, ImportMonster, ImportObject, ImportOptionalFeature, ImportPsionic, ImportRace,
    ImportRecipe, ImportReward, ImportSense, ImportSkill, ImportSpell, ImportSubclass,
    ImportSubclassFeature, ImportTrapHazard, ImportVariantRule, ImportVehicle,
};
use dllm_bindings::{
    DbConnection, seed_dnd_5_e_action, seed_dnd_5_e_background, seed_dnd_5_e_char_creation_option,
    seed_dnd_5_e_class, seed_dnd_5_e_class_feature, seed_dnd_5_e_condition, seed_dnd_5_e_cult_boon,
    seed_dnd_5_e_deck, seed_dnd_5_e_deity, seed_dnd_5_e_feat, seed_dnd_5_e_item,
    seed_dnd_5_e_language, seed_dnd_5_e_monster, seed_dnd_5_e_object,
    seed_dnd_5_e_optional_feature, seed_dnd_5_e_psionic, seed_dnd_5_e_race, seed_dnd_5_e_recipe,
    seed_dnd_5_e_reward, seed_dnd_5_e_sense, seed_dnd_5_e_skill, seed_dnd_5_e_spell,
    seed_dnd_5_e_subclass, seed_dnd_5_e_subclass_feature, seed_dnd_5_e_trap_hazard,
    seed_dnd_5_e_variant_rule, seed_dnd_5_e_vehicle,
};
use dllm_server::dnd5e::convert;

pub fn spell(conn: &DbConnection, spell: ImportSpell) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_spell(
            spell.name,
            spell.source,
            spell.level,
            convert::spell_school(spell.school),
            spell.ritual,
            spell.concentration,
            spell.description,
            spell.saving_throw.map(convert::ability),
        )
        .map_err(|err| err.to_string())
}

pub fn monster(conn: &DbConnection, monster: ImportMonster) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_monster(
            monster.name,
            monster.source,
            convert::creature_size(monster.size),
            convert::creature_type(monster.creature_type),
            monster.cr,
            monster.ac,
            monster.hp_average,
            monster.hp_formula,
            monster.speed_walk,
            monster.speed_fly,
            monster.speed_swim,
            monster.str_score,
            monster.dex_score,
            monster.con_score,
            monster.int_score,
            monster.wis_score,
            monster.cha_score,
            monster.description,
        )
        .map_err(|err| err.to_string())
}

pub fn item(conn: &DbConnection, item: ImportItem) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_item(
            item.name,
            item.source,
            convert::item_type(item.item_type),
            convert::item_rarity(item.rarity),
            item.weight,
            item.value,
            item.wondrous,
            item.attunement,
            item.description,
        )
        .map_err(|err| err.to_string())
}

pub fn feat(conn: &DbConnection, feat: ImportFeat) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_feat(
            feat.name,
            feat.source,
            feat.category.map(convert::feat_category),
            feat.prerequisite.map(convert::feat_prereq),
            feat.description,
        )
        .map_err(|err| err.to_string())
}

pub fn condition(conn: &DbConnection, condition: ImportCondition) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_condition(condition.name, condition.source, condition.description)
        .map_err(|err| err.to_string())
}

pub fn background(conn: &DbConnection, background: ImportBackground) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_background(
            background.name,
            background.source,
            background
                .skill_grants
                .into_iter()
                .map(convert::skill_grant)
                .collect(),
            background
                .tool_grants
                .into_iter()
                .map(convert::tool_grant)
                .collect(),
            background
                .language_grants
                .into_iter()
                .map(convert::language_grant)
                .collect(),
            background.description,
        )
        .map_err(|err| err.to_string())
}

pub fn race(conn: &DbConnection, race: ImportRace) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_race(
            race.name,
            race.source,
            convert::creature_size(race.size),
            convert::speed(race.speed),
            race.ability_grants
                .into_iter()
                .map(convert::ability_grant)
                .collect(),
            race.language_grants
                .into_iter()
                .map(convert::language_grant)
                .collect(),
            race.description,
        )
        .map_err(|err| err.to_string())
}

pub fn optional_feature(conn: &DbConnection, feature: ImportOptionalFeature) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_optional_feature(
            feature.name,
            feature.source,
            feature
                .feature_types
                .into_iter()
                .map(convert::optional_feature_type)
                .collect(),
            feature.prerequisite.map(convert::optional_feature_prereq),
            feature.description,
        )
        .map_err(|err| err.to_string())
}

pub fn action(conn: &DbConnection, action: ImportAction) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_action(action.name, action.source, action.time, action.description)
        .map_err(|err| err.to_string())
}

pub fn language(conn: &DbConnection, language: ImportLanguage) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_language(
            language.name,
            language.source,
            language.kind,
            language.script,
            language.origin,
            language.description,
        )
        .map_err(|err| err.to_string())
}

pub fn sense(conn: &DbConnection, sense: ImportSense) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_sense(sense.name, sense.source, sense.description)
        .map_err(|err| err.to_string())
}

pub fn skill(conn: &DbConnection, skill: ImportSkill) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_skill(
            skill.name,
            skill.source,
            convert::ability(skill.ability),
            skill.description,
        )
        .map_err(|err| err.to_string())
}

pub fn class(conn: &DbConnection, class: ImportClass) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_class(
            class.name,
            class.source,
            class.edition,
            class.hit_die,
            class
                .saving_throws
                .into_iter()
                .map(convert::ability)
                .collect(),
            class.spellcasting_ability.map(convert::ability),
            class.caster_progression.map(convert::caster_progression),
            class.prepared_spells_formula,
            class.prepared_spells_progression,
            class.cantrip_progression,
            class.class_features,
            class.subclass_title,
        )
        .map_err(|err| err.to_string())
}

pub fn subclass(conn: &DbConnection, subclass: ImportSubclass) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_subclass(
            subclass.name,
            subclass.short_name,
            subclass.source,
            subclass.class_name,
            subclass.class_source,
            subclass.edition,
            subclass.spellcasting_ability.map(convert::ability),
            subclass.caster_progression.map(convert::caster_progression),
            subclass.cantrip_progression,
            subclass.subclass_features,
        )
        .map_err(|err| err.to_string())
}

pub fn class_feature(conn: &DbConnection, feature: ImportClassFeature) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_class_feature(
            feature.name,
            feature.source,
            feature.class_name,
            feature.class_source,
            feature.level,
            feature.description,
        )
        .map_err(|err| err.to_string())
}

pub fn subclass_feature(conn: &DbConnection, feature: ImportSubclassFeature) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_subclass_feature(
            feature.name,
            feature.source,
            feature.class_name,
            feature.class_source,
            feature.subclass_short_name,
            feature.subclass_source,
            feature.level,
            feature.description,
        )
        .map_err(|err| err.to_string())
}

pub fn object(conn: &DbConnection, object: ImportObject) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_object(
            object.name,
            object.source,
            object.size,
            object.object_type,
            object.ac,
            object.hp,
            object.description,
            object.action_description,
        )
        .map_err(|err| err.to_string())
}

pub fn vehicle(conn: &DbConnection, vehicle: ImportVehicle) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_vehicle(
            vehicle.name,
            vehicle.source,
            vehicle.vehicle_type,
            vehicle.size,
            vehicle.terrain,
            vehicle.crew_capacity,
            vehicle.passenger_capacity,
            vehicle.pace,
            vehicle.ac,
            vehicle.hp,
            vehicle.description,
        )
        .map_err(|err| err.to_string())
}

pub fn deity(conn: &DbConnection, deity: ImportDeity) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_deity(
            deity.name,
            deity.source,
            deity.pantheon,
            deity.alignment,
            deity.category,
            deity.domains,
            deity.province,
            deity.title,
            deity.symbol,
            deity.description,
        )
        .map_err(|err| err.to_string())
}

pub fn reward(conn: &DbConnection, reward: ImportReward) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_reward(reward.name, reward.source, reward.kind, reward.description)
        .map_err(|err| err.to_string())
}

pub fn trap_hazard(conn: &DbConnection, trap_hazard: ImportTrapHazard) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_trap_hazard(
            trap_hazard.name,
            trap_hazard.source,
            trap_hazard.kind,
            trap_hazard.trap_hazard_type,
            trap_hazard.trigger,
            trap_hazard.effect,
            trap_hazard.countermeasures,
            trap_hazard.description,
        )
        .map_err(|err| err.to_string())
}

pub fn char_creation_option(
    conn: &DbConnection,
    option: ImportCharCreationOption,
) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_char_creation_option(
            option.name,
            option.source,
            option.option_types,
            option.description,
        )
        .map_err(|err| err.to_string())
}

pub fn psionic(conn: &DbConnection, psionic: ImportPsionic) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_psionic(
            psionic.name,
            psionic.source,
            psionic.kind,
            psionic.order,
            psionic.focus,
            psionic.description,
            psionic.modes,
        )
        .map_err(|err| err.to_string())
}

pub fn recipe(conn: &DbConnection, recipe: ImportRecipe) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_recipe(
            recipe.name,
            recipe.source,
            recipe.kind,
            recipe.dish_types,
            recipe.diet,
            recipe.serves,
            recipe.ingredients,
            recipe.instructions,
        )
        .map_err(|err| err.to_string())
}

pub fn cult_boon(conn: &DbConnection, cult_boon: ImportCultBoon) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_cult_boon(
            cult_boon.name,
            cult_boon.source,
            cult_boon.kind,
            cult_boon.subtype,
            cult_boon.goal,
            cult_boon.cultists,
            cult_boon.signature_spells,
            cult_boon.ability_text,
            cult_boon.description,
        )
        .map_err(|err| err.to_string())
}

pub fn deck(conn: &DbConnection, deck: ImportDeck) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_deck(deck.name, deck.source, deck.cards, deck.description)
        .map_err(|err| err.to_string())
}

pub fn variant_rule(conn: &DbConnection, rule: ImportVariantRule) -> Result<(), String> {
    conn.reducers
        .seed_dnd_5_e_variant_rule(rule.name, rule.source, rule.rule_type, rule.description)
        .map_err(|err| err.to_string())
}

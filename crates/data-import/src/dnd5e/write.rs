use crate::dnd5e::normalize::{
    ImportAction, ImportBackground, ImportClass, ImportClassFeature, ImportCondition, ImportFeat,
    ImportItem, ImportLanguage, ImportMonster, ImportOptionalFeature, ImportRace, ImportSense,
    ImportSkill, ImportSpell, ImportSubclass, ImportSubclassFeature,
};
use dllm_client::{
    DbConnection, seed_dnd_5_e_action, seed_dnd_5_e_background, seed_dnd_5_e_class,
    seed_dnd_5_e_class_feature, seed_dnd_5_e_condition, seed_dnd_5_e_feat, seed_dnd_5_e_item,
    seed_dnd_5_e_language, seed_dnd_5_e_monster, seed_dnd_5_e_optional_feature, seed_dnd_5_e_race,
    seed_dnd_5_e_sense, seed_dnd_5_e_skill, seed_dnd_5_e_spell, seed_dnd_5_e_subclass,
    seed_dnd_5_e_subclass_feature,
};
use spacetime::dnd5e::convert;

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

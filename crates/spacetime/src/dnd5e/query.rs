use crate::dnd5e::convert;
use dllm::dnd5e as dnd;
use dllm_client::{
    DbConnection, dnd_5_e_item_table::Dnd5EItemTableAccess,
    dnd_5_e_monster_table::Dnd5EMonsterTableAccess, dnd_5_e_spell_table::Dnd5ESpellTableAccess,
};
use spacetimedb_sdk::{DbContext, Table};
use std::str::FromStr;
use tracing::info;

#[derive(Clone, Debug, Default)]
pub struct SpellQuery {
    pub name: Option<String>,
    pub level: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct SpellSummary {
    pub source: String,
    pub name: String,
    pub level: u8,
    pub school: dnd::SpellSchool,
    pub description: String,
}

#[derive(Clone, Debug, Default)]
pub struct MonsterQuery {
    pub name: Option<String>,
    pub cr: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MonsterSummary {
    pub source: String,
    pub name: String,
    pub cr: String,
    pub size: dnd::CreatureSize,
    pub creature_type: dnd::CreatureType,
    pub hp_average: u16,
    pub ac: u8,
}

#[derive(Clone, Debug, Default)]
pub struct ItemQuery {
    pub name: Option<String>,
    pub rarity: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ItemSummary {
    pub source: String,
    pub name: String,
    pub rarity: dnd::ItemRarity,
    pub item_type: dnd::ItemType,
}

pub fn query_spells(conn: &DbConnection, query: SpellQuery) -> Vec<SpellSummary> {
    subscribe_and_wait(conn, ["SELECT * FROM dnd_5_e_spell"]);
    let count = conn.db.dnd_5_e_spell().count();
    info!(count, "subscribed to spells");

    let name = query.name.map(|value| value.to_lowercase());

    conn.db
        .dnd_5_e_spell()
        .iter()
        .filter(|spell| {
            let matches_name = name
                .as_ref()
                .is_none_or(|value| spell.name.to_lowercase().contains(value));
            let matches_level = query.level.is_none_or(|level| spell.level == level);
            matches_name && matches_level
        })
        .map(|spell| SpellSummary {
            source: spell.source.clone(),
            name: spell.name.clone(),
            level: spell.level,
            school: convert::spell_school_from_client(spell.school),
            description: spell.description.clone(),
        })
        .collect()
}

pub fn query_monsters(conn: &DbConnection, query: MonsterQuery) -> Vec<MonsterSummary> {
    subscribe_and_wait(conn, ["SELECT * FROM dnd_5_e_monster"]);
    let count = conn.db.dnd_5_e_monster().count();
    info!(count, "subscribed to monsters");

    let name = query.name.map(|value| value.to_lowercase());

    conn.db
        .dnd_5_e_monster()
        .iter()
        .filter(|monster| {
            let matches_name = name
                .as_ref()
                .is_none_or(|value| monster.name.to_lowercase().contains(value));
            let matches_cr = query.cr.as_ref().is_none_or(|value| monster.cr == *value);
            matches_name && matches_cr
        })
        .map(|monster| MonsterSummary {
            source: monster.source.clone(),
            name: monster.name.clone(),
            cr: monster.cr.clone(),
            size: convert::creature_size_from_client(monster.size),
            creature_type: convert::creature_type_from_client(monster.creature_type),
            hp_average: monster.hp_average,
            ac: monster.ac,
        })
        .collect()
}

pub fn query_items(conn: &DbConnection, query: ItemQuery) -> Vec<ItemSummary> {
    subscribe_and_wait(conn, ["SELECT * FROM dnd_5_e_item"]);
    let count = conn.db.dnd_5_e_item().count();
    info!(count, "subscribed to items");

    let name = query.name.map(|value| value.to_lowercase());
    let rarity = query
        .rarity
        .and_then(|value| dnd::ItemRarity::from_str(&value).ok());

    conn.db
        .dnd_5_e_item()
        .iter()
        .filter(|item| {
            let matches_name = name
                .as_ref()
                .is_none_or(|value| item.name.to_lowercase().contains(value));
            let matches_rarity = rarity
                .as_ref()
                .is_none_or(|value| convert::item_rarity_from_client(item.rarity) == *value);
            matches_name && matches_rarity
        })
        .map(|item| ItemSummary {
            source: item.source.clone(),
            name: item.name.clone(),
            rarity: convert::item_rarity_from_client(item.rarity),
            item_type: convert::item_type_from_client(item.item_type),
        })
        .collect()
}

fn subscribe_and_wait<const N: usize>(conn: &DbConnection, sql: [&str; N]) {
    conn.subscription_builder().subscribe(sql);

    let _handle = conn.run_threaded();
    std::thread::sleep(std::time::Duration::from_secs(2));
}

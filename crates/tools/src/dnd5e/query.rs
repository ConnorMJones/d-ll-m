use dllm_bindings::DbConnection;
use dllm_server::dnd5e::query::{self as db_query, ItemQuery, MonsterQuery, SpellQuery};

pub fn query_spells(conn: &DbConnection, name_filter: Option<String>, level_filter: Option<u8>) {
    let spells = db_query::query_spells(
        conn,
        SpellQuery {
            name: name_filter.clone(),
            level: level_filter,
        },
    );

    for spell in &spells {
        println!(
            "[{}] {} (Level {}, {:?})",
            spell.source, spell.name, spell.level, spell.school
        );
        if name_filter.is_some() {
            let desc_preview: String = spell.description.chars().take(200).collect();
            println!("  {}", desc_preview);
        }
    }

    println!("\nFound {} spells.", spells.len());
}

pub fn query_monsters(conn: &DbConnection, name_filter: Option<String>, cr_filter: Option<String>) {
    let monsters = db_query::query_monsters(
        conn,
        MonsterQuery {
            name: name_filter,
            cr: cr_filter,
        },
    );

    for monster in &monsters {
        println!(
            "[{}] {} (CR {}, {:?} {:?}, HP {}, AC {})",
            monster.source,
            monster.name,
            monster.cr,
            monster.size,
            monster.creature_type,
            monster.hp_average,
            monster.ac
        );
    }

    println!("\nFound {} monsters.", monsters.len());
}

pub fn query_items(
    conn: &DbConnection,
    name_filter: Option<String>,
    rarity_filter: Option<String>,
) {
    let items = db_query::query_items(
        conn,
        ItemQuery {
            name: name_filter,
            rarity: rarity_filter,
        },
    );

    for item in &items {
        println!(
            "[{}] {} ({:?}, {:?})",
            item.source, item.name, item.rarity, item.item_type
        );
    }

    println!("\nFound {} items.", items.len());
}

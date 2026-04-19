use super::support::read_json_file;
use crate::dnd5e::normalize::{
    normalize_char_creation_option, normalize_cult_boon, normalize_deck, normalize_deity,
    normalize_object, normalize_psionic, normalize_recipe, normalize_reward, normalize_trap_hazard,
    normalize_variant_rule, normalize_vehicle,
};
use crate::dnd5e::report::SectionReport;
use crate::dnd5e::types::{
    CharCreationOptionFile, CultsBoonsFile, DeckFile, DeityFile, ObjectFile, PsionicFile,
    RecipeFile, RewardFile, TrapHazardFile, VariantRuleFile, VehicleFile,
};
use crate::dnd5e::write;
use dllm_bindings::DbConnection;
use std::path::Path;
use tracing::info;

pub fn seed_objects(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("objects.json");
    let mut report = SectionReport::new("objects");

    if !path.exists() {
        info!("objects.json not found, skipping");
        return report;
    }

    info!("reading objects");

    let Some(file) = read_json_file::<ObjectFile>(&path, &mut report) else {
        return report;
    };

    for raw_object in file.object {
        report.note_seen();
        let object = normalize_object(raw_object, &mut report);
        let item_name = format!("{} [{}]", object.name, object.source);

        match write::object(conn, object) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded objects");
    report
}

pub fn seed_vehicles(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("vehicles.json");
    let mut report = SectionReport::new("vehicles");

    if !path.exists() {
        info!("vehicles.json not found, skipping");
        return report;
    }

    info!("reading vehicles");

    let Some(file) = read_json_file::<VehicleFile>(&path, &mut report) else {
        return report;
    };

    for raw_vehicle in file.vehicle {
        report.note_seen();
        let vehicle = normalize_vehicle(raw_vehicle, &mut report);
        let item_name = format!("{} [{}]", vehicle.name, vehicle.source);

        match write::vehicle(conn, vehicle) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded vehicles");
    report
}

pub fn seed_deities(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("deities.json");
    let mut report = SectionReport::new("deities");

    if !path.exists() {
        info!("deities.json not found, skipping");
        return report;
    }

    info!("reading deities");

    let Some(file) = read_json_file::<DeityFile>(&path, &mut report) else {
        return report;
    };

    for raw_deity in file.deity {
        report.note_seen();
        let deity = normalize_deity(raw_deity, &mut report);
        let item_name = format!("{} [{}]", deity.name, deity.source);

        match write::deity(conn, deity) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded deities");
    report
}

pub fn seed_rewards(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("rewards.json");
    let mut report = SectionReport::new("rewards");

    if !path.exists() {
        info!("rewards.json not found, skipping");
        return report;
    }

    info!("reading rewards");

    let Some(file) = read_json_file::<RewardFile>(&path, &mut report) else {
        return report;
    };

    for raw_reward in file.reward {
        report.note_seen();
        let reward = normalize_reward(raw_reward, &mut report);
        let item_name = format!("{} [{}]", reward.name, reward.source);

        match write::reward(conn, reward) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded rewards");
    report
}

pub fn seed_trap_hazards(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("trapshazards.json");
    let mut report = SectionReport::new("trap_hazards");

    if !path.exists() {
        info!("trapshazards.json not found, skipping");
        return report;
    }

    info!("reading trap hazards");

    let Some(file) = read_json_file::<TrapHazardFile>(&path, &mut report) else {
        return report;
    };

    for raw_trap in file.trap {
        report.note_seen();
        let trap = normalize_trap_hazard(raw_trap, "trap", &mut report);
        let item_name = format!("{} [{}]", trap.name, trap.source);

        match write::trap_hazard(conn, trap) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    for raw_hazard in file.hazard {
        report.note_seen();
        let hazard = normalize_trap_hazard(raw_hazard, "hazard", &mut report);
        let item_name = format!("{} [{}]", hazard.name, hazard.source);

        match write::trap_hazard(conn, hazard) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded trap hazards");
    report
}

pub fn seed_char_creation_options(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("charcreationoptions.json");
    let mut report = SectionReport::new("char_creation_options");

    if !path.exists() {
        info!("charcreationoptions.json not found, skipping");
        return report;
    }

    info!("reading char creation options");

    let Some(file) = read_json_file::<CharCreationOptionFile>(&path, &mut report) else {
        return report;
    };

    for raw_option in file.charoption {
        report.note_seen();
        let option = normalize_char_creation_option(raw_option, &mut report);
        let item_name = format!("{} [{}]", option.name, option.source);

        match write::char_creation_option(conn, option) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded char creation options");
    report
}

pub fn seed_psionics(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("psionics.json");
    let mut report = SectionReport::new("psionics");

    if !path.exists() {
        info!("psionics.json not found, skipping");
        return report;
    }

    info!("reading psionics");

    let Some(file) = read_json_file::<PsionicFile>(&path, &mut report) else {
        return report;
    };

    for raw_psionic in file.psionic {
        report.note_seen();
        let psionic = normalize_psionic(raw_psionic, &mut report);
        let item_name = format!("{} [{}]", psionic.name, psionic.source);

        match write::psionic(conn, psionic) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded psionics");
    report
}

pub fn seed_recipes(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("recipes.json");
    let mut report = SectionReport::new("recipes");

    if !path.exists() {
        info!("recipes.json not found, skipping");
        return report;
    }

    info!("reading recipes");

    let Some(file) = read_json_file::<RecipeFile>(&path, &mut report) else {
        return report;
    };

    for raw_recipe in file.recipe {
        report.note_seen();
        let recipe = normalize_recipe(raw_recipe, &mut report);
        let item_name = format!("{} [{}]", recipe.name, recipe.source);

        match write::recipe(conn, recipe) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded recipes");
    report
}

pub fn seed_cults_boons(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("cultsboons.json");
    let mut report = SectionReport::new("cults_boons");

    if !path.exists() {
        info!("cultsboons.json not found, skipping");
        return report;
    }

    info!("reading cults and boons");

    let Some(file) = read_json_file::<CultsBoonsFile>(&path, &mut report) else {
        return report;
    };

    for raw_cult in file.cult {
        report.note_seen();
        let cult = normalize_cult_boon(raw_cult, "cult", &mut report);
        let item_name = format!("{} [{}]", cult.name, cult.source);

        match write::cult_boon(conn, cult) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    for raw_boon in file.boon {
        report.note_seen();
        let boon = normalize_cult_boon(raw_boon, "boon", &mut report);
        let item_name = format!("{} [{}]", boon.name, boon.source);

        match write::cult_boon(conn, boon) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded cults and boons");
    report
}

pub fn seed_decks(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("decks.json");
    let mut report = SectionReport::new("decks");

    if !path.exists() {
        info!("decks.json not found, skipping");
        return report;
    }

    info!("reading decks");

    let Some(file) = read_json_file::<DeckFile>(&path, &mut report) else {
        return report;
    };

    for raw_deck in file.deck {
        report.note_seen();
        let deck = normalize_deck(raw_deck, &mut report);
        let item_name = format!("{} [{}]", deck.name, deck.source);

        match write::deck(conn, deck) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded decks");
    report
}

pub fn seed_variant_rules(conn: &DbConnection, data_dir: &Path) -> SectionReport {
    let path = data_dir.join("variantrules.json");
    let mut report = SectionReport::new("variant_rules");

    if !path.exists() {
        info!("variantrules.json not found, skipping");
        return report;
    }

    info!("reading variant rules");

    let Some(file) = read_json_file::<VariantRuleFile>(&path, &mut report) else {
        return report;
    };

    for raw_rule in file.variantrule {
        report.note_seen();
        let rule = normalize_variant_rule(raw_rule, &mut report);
        let item_name = format!("{} [{}]", rule.name, rule.source);

        match write::variant_rule(conn, rule) {
            Ok(()) => report.imported(),
            Err(err) => report.failed(item_name, format!("reducer call failed: {err}")),
        }
    }

    info!(count = report.imported, "seeded variant rules");
    report
}

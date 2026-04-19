use clap::Parser;
use dllm_tools::*;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Seed { data_dir } => dnd5e::seed(&data_dir),
        Commands::QuerySpells { name, level } => dnd5e::query_spells(name, level),
        Commands::QueryMonsters { name, cr } => dnd5e::query_monsters(name, cr),
        Commands::QueryItems { name, rarity } => dnd5e::query_items(name, rarity),
    }
}

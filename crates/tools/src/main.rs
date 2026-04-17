use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod convert;
mod dnd5e;

#[derive(Parser)]
#[command(name = "dllm-tools")]
#[command(about = "D-LL-M administrative tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Seed D&D 5e data from 5e.tools JSON files
    Seed {
        #[arg(long, default_value = "ttrpg/dnd5e/data")]
        data_dir: PathBuf,
    },
    /// Query spells from the database
    QuerySpells {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        level: Option<u8>,
    },
    /// Query monsters from the database
    QueryMonsters {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        cr: Option<String>,
    },
    /// Query items from the database
    QueryItems {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        rarity: Option<String>,
    },
}

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

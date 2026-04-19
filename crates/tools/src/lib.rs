pub mod dnd5e;
pub use data_import;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dllm-tools")]
#[command(about = "D-LL-M administrative tools")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

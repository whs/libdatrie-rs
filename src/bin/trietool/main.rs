mod command_add;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long, default_value = ".", help = "Trie directory")]
    path: PathBuf,

    #[arg(help = "Trie name")]
    trie: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Add WORD with DATA to trie")]
    Add { word: String, data: String },
    #[command(about = "Add words and data listed in LIST_FILE to trie")]
    AddList {
        list_file: PathBuf,

        #[arg(long, help = "specify character encoding of LIST_FILE")]
        encoding: Option<String>,
    },
    #[command(about = "Delete WORD from trie")]
    Delete { word: String },
    #[command(about = "Delete words listed in LIST_FILE from trie")]
    DeleteList {
        list_file: PathBuf,

        #[arg(long, help = "specify character encoding of LIST_FILE")]
        encoding: Option<String>,
    },
    #[command(about = "Query WORD data from trie")]
    Query { word: String },
    #[command(about = "List all words in trie")]
    List {},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        ref Add => command_add::command(cli),
        _ => todo!(),
    }
}

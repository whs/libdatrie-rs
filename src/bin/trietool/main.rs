use std::path::PathBuf;

use clap::{Parser, Subcommand};

use datrie::CTrieData;

use crate::utils::{load_trie, AutoSaveTrie};

mod command_add;
mod command_add_list;
mod command_delete;
mod command_delete_list;
mod command_list;
mod command_query;
mod utils;

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
    Add {
        #[arg(trailing_var_arg=true, value_names=["WORD", "DATA"])]
        pairs: Vec<String>,
    },
    #[command(about = "Add words and data listed in LIST_FILE to trie")]
    AddList {
        list_file: PathBuf,

        #[arg(short, long, help = "specify character encoding of LIST_FILE")]
        encoding: Option<String>,
    },
    #[command(about = "Delete WORD from trie")]
    Delete { words: Vec<String> },
    #[command(about = "Delete words listed in LIST_FILE from trie")]
    DeleteList {
        list_file: PathBuf,

        #[arg(short, long, help = "specify character encoding of LIST_FILE")]
        encoding: Option<String>,
    },
    #[command(about = "Query WORD data from trie")]
    Query { word: String },
    #[command(about = "List all words in trie")]
    List {},
}

pub struct Context {
    trie: AutoSaveTrie<Option<CTrieData>>,
}

fn main() {
    let cli = Cli::parse();
    let trie = load_trie(&cli);
    let mut context = Context { trie };

    match cli.command {
        Commands::Add { pairs } => command_add::add(&mut context, pairs),
        Commands::AddList {
            list_file,
            encoding,
        } => command_add_list::add_list(&mut context, list_file, encoding),
        Commands::Delete { words } => command_delete::delete(&mut context, words),
        Commands::DeleteList {
            list_file,
            encoding,
        } => command_delete_list::delete_list(&mut context, list_file, encoding),
        Commands::Query { word } => command_query::query(&context, word),
        Commands::List {} => command_list::list(&context),
    }
}

use crate::utils::load_trie;
use crate::Cli;

pub fn command(cli: Cli) {
    let trie = load_trie(&cli);
}

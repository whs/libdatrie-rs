use crate::utils::str_to_ucs4le;
use crate::Context;
use datrie::{CTrieData, TRIE_DATA_ERROR};
use std::process::exit;

pub fn add(context: &mut Context, words: Vec<String>) {
    if words.len() % 2 != 0 {
        println!("Missing data");
        exit(1);
    }

    let words = words.chunks_exact(2);

    for pair in words {
        let word = &pair[0];
        let data = pair[1]
            .parse::<i32>()
            .map(|v| CTrieData(v))
            .unwrap_or(TRIE_DATA_ERROR);

        // conv_to_alpha
        let key = str_to_ucs4le(word);
        if !context.trie.store(&key, Some(data)) {
            eprintln!("Failed to add entry '{}' with data {}", word, data.0);
        }
    }
}

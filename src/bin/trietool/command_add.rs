use std::ops::Deref;
use std::process::exit;

use datrie::{AsAlphaChar, CTrieData, TRIE_DATA_ERROR};

use crate::Context;

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

        let key = word.deref().as_alphachar();
        if !context.trie.store(&key, Some(data)) {
            eprintln!("Failed to add entry '{}' with data {}", word, data.0);
        }
    }
}

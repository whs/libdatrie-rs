use std::ops::Deref;

use datrie::{AlphaCharToString, TRIE_DATA_ERROR};

use crate::Context;

pub fn list(context: &Context) {
    for (key, value) in context.trie.iter() {
        println!(
            "{}\t{}",
            key.deref().ac_to_string().unwrap(),
            value.flatten().unwrap_or(TRIE_DATA_ERROR).0
        )
    }
}

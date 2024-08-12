use crate::Context;
use datrie::{AsAlphaChar, TRIE_DATA_ERROR};
use std::ops::Deref;

pub fn query(context: &Context, key: String) {
    let alphachars = key.deref().as_alphachar();
    let out = context.trie.retrieve(&alphachars).copied();
    match out {
        Some(data) => println!("{}", data.unwrap_or(TRIE_DATA_ERROR).0),
        None => eprintln!("query: Key '{}' not found.", key),
    }
}

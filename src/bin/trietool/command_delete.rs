use std::ops::Deref;

use datrie::AsAlphaChar;

use crate::Context;

pub fn delete(context: &mut Context, words: Vec<String>) {
    for word in words {
        let alphachars = word.deref().as_alphachar();
        if !context.trie.delete(&alphachars) {
            eprintln!("No entry '{}'. Not deleted", word);
        }
    }
}

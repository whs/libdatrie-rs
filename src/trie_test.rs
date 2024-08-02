use std::collections::{BTreeSet, HashMap};
use std::io::Cursor;

use crate::alpha_map::AlphaMap;
use crate::testutils::*;
use crate::trie::Trie;
use crate::types::{AlphaChar, AsAlphaChar};
use crate::types_c::TrieData;

// Ported from test_null_trie.c
#[test]
fn test_null_trie() {
    println!("Preparing empty trie");
    let trie = en_trie_new();
    assert_eq!(trie.iter().count(), 0, "Trie should not have entry");
}

// Ported from test_byte_alpha.c
#[test]
fn test_byte_alpha() {
    println!("Preparing alpha map");
    let mut alpha_map = AlphaMap::default();
    alpha_map.add_range(0x00..=0xff);

    println!("Preparing trie");
    let mut trie = Trie::new(alpha_map);

    println!("Storing key to test trie");
    let key = [0xff, 0xff, 0];
    assert!(trie.store(&key, 1), "Failed to store key to test trie");

    println!("Retrieving data from test trie");
    let data = trie
        .retrieve(&key)
        .expect("Failed to retrieve key from test trie");
    assert_eq!(*data, 1, "Incorrect TrieData received");
}

// Ported from test_byte_list.c
#[test]
fn test_byte_list() {
    println!("Preparing alpha map");
    let mut alpha_map = AlphaMap::default();
    alpha_map.add_range(0x00..=0xff);

    println!("Preparing trie");
    let mut trie = Trie::new(alpha_map);

    println!("Storing entries to test trie");
    let source: HashMap<Vec<AlphaChar>, TrieData> = HashMap::from([
        (vec!['1'.into(), '2'.into(), 0], TrieData(1)),
        (vec!['1'.into(), '2'.into(), '3'.into(), 0], TrieData(2)),
    ]);
    for (key, value) in &source {
        assert!(trie.store(key, *value), "Failed to store {:?}", key);
    }

    let mut found_set: BTreeSet<&Vec<AlphaChar>> = source.keys().collect();

    println!("Iterating trie");
    for (key, _) in trie.iter() {
        found_set.remove(&key);
    }

    assert_eq!(found_set.len(), 0);
}

// Ported from test_file.c and test_serialization.c
#[test]
fn test_serialize() {
    println!("Preparing trie");
    let mut trie = en_trie_new();

    // add/remove some words
    for word in DICT {
        assert!(
            trie.store(&word.as_alphachar(), 1),
            "Failed to store {}",
            word
        );
    }

    // save and close
    println!("Serializing trie");
    let mut buf = Vec::new();
    trie.serialize(&mut buf).expect("Failed to serialize trie");

    // reload from file
    println!("Reloading trie from serialized data");
    let trie = Trie::from_reader(&mut Cursor::new(&buf))
        .expect("Failed to reload trie from serialized state");

    // enumerate & check
    println!("Checking trie contents");
    assert_dict_complete(&trie);

    assert_eq!(
        buf.len(),
        trie.serialized_size(),
        "Serialized actual length is not equal to estimated size"
    );
}

// Ported from test_nonalpha.c
#[test]
fn test_nonalpha() {
    println!("Preparing trie");
    let mut trie = en_trie_new();

    // store
    println!("Adding data to trie");
    for word in DICT {
        assert!(
            trie.store(&word.as_alphachar(), 1),
            "Failed to store {}",
            word
        );
    }

    // test storing keys with non-alphabet chars
    for word in ["a6acus", "a5acus"] {
        assert_eq!(
            trie.retrieve(&word.as_alphachar()),
            None,
            "Trie has false data on {}",
            word
        );
        assert!(
            !trie.store(&word.as_alphachar(), 1),
            "Successfully stored {} which should not happen",
            word
        );
    }

    assert_dict_complete(&trie);
}

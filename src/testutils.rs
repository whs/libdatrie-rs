use std::collections::BTreeSet;
use std::ops::Deref;

use crate::alpha_map::AlphaMap;
use crate::trie::Trie;
use crate::types::AlphaCharToString;

pub fn en_alpha_map_new() -> AlphaMap {
    let mut en_map = AlphaMap::default();
    en_map.add_range(0x0061..=0x007a);

    en_map
}

pub fn en_trie_new() -> Trie {
    let en_map = en_alpha_map_new();
    let en_trie = Trie::new(en_map);
    en_trie
}

pub static DICT: &[&str] = &[
    "a",
    "abacus",
    "abandon",
    "accident",
    "accredit",
    "algorithm",
    "ammonia",
    "angel",
    "angle",
    "azure",
    "bat",
    "bet",
    "best",
    "home",
    "house",
    "hut",
    "king",
    "kite",
    "name",
    "net",
    "network",
    "nut",
    "nutshell",
    "quality",
    "quantum",
    "quantity",
    "quartz",
    "quick",
    "quiz",
    "run",
    "tape",
    "test",
    "what",
    "when",
    "where",
    "which",
    "who",
    "why",
    "zebra",
];

pub fn assert_dict_complete(trie: &Trie) {
    let mut found_set = BTreeSet::from_iter(DICT.iter().copied());
    for (key, data) in trie.iter() {
        assert_eq!(data.expect("Failed to get data"), 1);

        let key_str = key.deref().ac_to_string().unwrap();
        found_set.remove(key_str.deref());
    }

    if found_set.len() != 0 {
        for missing in &found_set {
            println!("Missing key: {}", missing)
        }
    }
    assert_eq!(found_set.len(), 0)
}

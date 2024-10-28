#![no_main]

use arbitrary::Arbitrary;
use datrie::{AlphaChar, AlphaMap, Trie, ALPHA_CHAR_ERROR};
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Arbitrary, Debug)]
struct Input {
    pub am_range: RangeInclusive<AlphaChar>,
    pub data: HashMap<Vec<AlphaChar>, Option<i32>>,
}

fuzz_target!(|input: Input| {
    if input.am_range.contains(&ALPHA_CHAR_ERROR) {
        // This should be banned in add_range?
        return;
    }
    if input.am_range.clone().count() > (u8::MAX - 1) as usize {
        // This should be banned in add_range?
        return;
    }

    let mut am = AlphaMap::default();
    am.add_range(input.am_range.clone());
    let mut trie = Trie::<Option<i32>>::new(am);

    for item in input.data.iter() {
        // validate that the ac doesn't have inner null bytes, except the last byte must be 0
        match item.0.iter().position(|v| *v == 0) {
            Some(v) if v == item.0.len() - 1 => {}
            _ => return,
        };
        // validate that the ac is in the range
        match item.0.iter().position(|v| !input.am_range.contains(v)) {
            Some(_) => return,
            _ => {}
        }

        trie.store(item.0, *item.1);
    }

    for item in trie.iter() {
        // Check that the key exists
        let value = input.data.get(&item.0).expect(
            format!(
                "got a pair '{:?}' from the trie, but it is not in the initial input",
                item.0
            )
            .as_str(),
        );
        assert_eq!(*item.1.unwrap(), *value)
    }

    for item in input.data.iter() {
        let value = trie
            .retrieve(item.0)
            .expect(format!("an input key '{:?}' is missing from the tree", item.0).as_str());
        assert_eq!(*item.1, *value);
    }
});

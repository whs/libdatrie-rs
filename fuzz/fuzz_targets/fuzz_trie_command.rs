#![no_main]

use arbitrary::Arbitrary;
use datrie::{AlphaChar, AlphaMap, Trie, ALPHA_CHAR_ERROR};
use libfuzzer_sys::fuzz_target;
use std::hint::black_box;
use std::io::Cursor;
use std::ops::RangeInclusive;

#[derive(Arbitrary, Debug)]
struct Input {
    pub am_range: RangeInclusive<AlphaChar>,
    pub commands: Vec<Command>,
}

#[derive(Arbitrary, Debug)]
enum Command {
    Store {
        key: Vec<AlphaChar>,
        data: Option<i32>,
    },
    StoreIfAbsent {
        key: Vec<AlphaChar>,
        data: Option<i32>,
    },
    Root,
    Retrieve {
        key: Vec<AlphaChar>,
    },
    Delete {
        key: Vec<AlphaChar>,
    },
    SerdeTest,
}

fn validate_key(input: &Input, key: &Vec<AlphaChar>) -> bool {
    match key.iter().position(|v| *v == 0) {
        Some(v) if v == key.len() - 1 => {}
        _ => return false,
    };
    // validate that the ac is in the range
    match key.iter().position(|v| !input.am_range.contains(v)) {
        Some(_) => return false,
        _ => {}
    }

    true
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

    for command in input.commands.iter() {
        match command {
            Command::Store { key, data } => {
                if !validate_key(&input, key) {
                    return;
                }
                trie.store(key, *data);
            }
            Command::StoreIfAbsent { key, data } => {
                if !validate_key(&input, key) {
                    return;
                }
                trie.store(key, *data);
            }
            Command::Root => {
                black_box(trie.root());
            }
            Command::Retrieve { key } => {
                if !validate_key(&input, key) {
                    return;
                }
                trie.retrieve(key);
            }
            Command::Delete { key } => {
                if !validate_key(&input, key) {
                    return;
                }
                trie.delete(key);
            }
            Command::SerdeTest => {
                let mut buf: Vec<u8> = Vec::new();
                trie.serialize(&mut buf).unwrap();

                let mut buf_cursor = Cursor::new(&buf);
                let mut new_trie = Trie::<Option<i32>>::from_reader(&mut buf_cursor).unwrap();

                let mut new_buf: Vec<u8> = Vec::new();
                new_trie.serialize(&mut new_buf).unwrap();

                assert_eq!(buf, new_buf);
            }
        }
    }
});

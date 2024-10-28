#![no_main]

use datrie::Trie;
use libfuzzer_sys::fuzz_target;
use std::hint::black_box;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut buf = Cursor::new(data);
    let trie = match Trie::<i32>::from_reader(&mut buf) {
        Ok(v) => v,
        Err(_) => return,
    };

    for item in trie.iter() {
        black_box(item);
    }
});

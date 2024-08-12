use datrie::AsAlphaChar;
use iconv::Iconv;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::PathBuf;

use crate::utils::IconvExt;
use crate::Context;

pub fn delete_list(context: &mut Context, list_file: PathBuf, encoding: Option<String>) {
    let mut encoder =
        encoding.map(|v| Iconv::new(&v, "UTF-8").expect("Unable to create iconv encoder"));

    let input = BufReader::new(File::open(&list_file).expect("Cannot open input file"));

    for line in input.split('\n' as u8) {
        let line = line.unwrap();

        let key_str = match encoder.as_mut() {
            Some(encoder) => {
                let data = encoder
                    .decode(line.trim_ascii())
                    .expect("Failed to decode data");
                unsafe { String::from_utf8_unchecked(data) }
            }
            // XXX: In libthai this is current locale's encoding. Instead, we only support UTF-8
            None => String::from_utf8_lossy(line.trim_ascii()).to_string(),
        };
        let encoded_key = key_str.deref().as_alphachar();

        if !context.trie.delete(&encoded_key) {
            eprintln!("No entry '{}'. Not deleted", key_str);
        }
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::PathBuf;

use datrie::{AsAlphaChar, CTrieData, TRIE_DATA_ERROR};
use iconv::Iconv;

use crate::utils::IconvExt;
use crate::Context;

pub fn add_list(context: &mut Context, list_file: PathBuf, encoding: Option<String>) {
    let split_pattern = regex::bytes::Regex::new(r"[\t,]").unwrap();
    let mut encoder = match encoding {
        Some(v) if v.to_lowercase() == "utf-8" => None,
        Some(v) => Some(Iconv::new(&v, "UTF-8").expect("Unable to create iconv encoder")),
        None => None,
    };

    let input = BufReader::new(File::open(&list_file).expect("Cannot open input file"));

    // the file format should be CSV or TSV-ish
    for line in input.split('\n' as u8) {
        let line = line.unwrap();
        let split: Vec<&[u8]> = split_pattern.splitn(line.trim_ascii(), 2).collect();

        let key = split[0];
        let key_str = match encoder.as_mut() {
            Some(encoder) => {
                let data = encoder.decode(key).expect("Failed to decode data");
                unsafe { String::from_utf8_unchecked(data) }
            }
            // XXX: In libdatrie this is current locale's encoding. Instead, we only support UTF-8
            None => String::from_utf8_lossy(key).to_string(),
        };
        let encoded_key = key_str.deref().as_alphachar();

        let data = split
            .get(1)
            .map(|v| {
                String::from_utf8_lossy(v)
                    .parse::<i32>()
                    .map(|v| CTrieData(v))
                    .ok()
            })
            .flatten()
            .unwrap_or(TRIE_DATA_ERROR);

        if !context.trie.store(&encoded_key, Some(data)) {
            eprintln!("Failed to add entry '{}' with data {}", key_str, data.0);
        }
    }
}

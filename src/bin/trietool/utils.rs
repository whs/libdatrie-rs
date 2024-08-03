use std::{io, iter};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use regex::Regex;

use datrie::{CTrieData, TrieDeserializable, TrieSerializable};
use datrie::alpha_map::AlphaMap;
use datrie::trie::Trie;
use datrie::types::AlphaChar;

use crate::Cli;

pub fn load_trie(cli: &Cli) -> AutoSaveTrie<Option<CTrieData>> {
    let trie_path = cli.path.join(format!("{}.tri", cli.trie));

    if let Ok(trie) = AutoSaveTrie::from_file(&trie_path) {
        return trie;
    }

    let abm_path = cli.path.join(format!("{}.abm", cli.trie));
    let mut reader = BufReader::new(File::open(&abm_path).expect(&format!(
        "Cannot open alphabet map file {}",
        abm_path.display()
    )));
    let mut alpha_map = AlphaMap::default();
    load_abm(&mut alpha_map, &mut reader).expect("Fail to load alphabet map");

    AutoSaveTrie::new(trie_path, alpha_map)
}

pub fn load_abm<R: Read>(alpha_map: &mut AlphaMap, stream: &mut BufReader<R>) -> io::Result<()> {
    // format: [begin_char,end_char]
    // where begin_char and end_char is in hex values
    let pattern = Regex::new(r"\s*\[\s*([0-9a-f]+)\s*,\s*([0-9a-f]+)\s*]").unwrap();

    for line in stream.lines() {
        if let Some(captures) = pattern.captures(&line.unwrap()) {
            let begin = captures.get(1).unwrap().as_str();
            let end = captures.get(2).unwrap().as_str();

            let Ok(begin_ac) = AlphaChar::from_str_radix(begin, 16) else {
                continue;
            };
            let Ok(end_ac) = AlphaChar::from_str_radix(end, 16) else {
                continue;
            };

            if begin_ac > end_ac {
                eprintln!("Range begin ({}) > range end ({}", begin_ac, end_ac);
                continue;
            }

            alpha_map.add_range(begin_ac..=end_ac);
        }
    }

    Ok(())
}

pub fn str_to_ucs4le(s: &str) -> Vec<AlphaChar> {
    // This also add the null byte
    s.chars()
        .map(|c| c as u32 as AlphaChar)
        .chain(iter::once(0))
        .collect()
}

pub struct AutoSaveTrie<T: Default + Clone + TrieSerializable + TrieDeserializable> {
    path: PathBuf,
    trie: Trie<T>,
}

impl<T: Default + Clone + TrieSerializable + TrieDeserializable> AutoSaveTrie<T> {
    pub fn new<P: AsRef<Path>>(path: P, alpha_map: AlphaMap) -> Self {
        AutoSaveTrie {
            path: path.as_ref().to_path_buf(),
            trie: Trie::new(alpha_map),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut fp = BufReader::new(File::open(&path)?);
        let trie = Trie::from_reader(&mut fp)?;
        Ok(AutoSaveTrie {
            path: path.as_ref().to_path_buf(),
            trie,
        })
    }
}

impl<T: Default + Clone + TrieSerializable + TrieDeserializable> Deref for AutoSaveTrie<T> {
    type Target = Trie<T>;

    fn deref(&self) -> &Self::Target {
        &self.trie
    }
}

impl<T: Default + Clone + TrieSerializable + TrieDeserializable> DerefMut for AutoSaveTrie<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.trie
    }
}

impl<T: Default + Clone + TrieSerializable + TrieDeserializable> Drop for AutoSaveTrie<T> {
    fn drop(&mut self) {
        if self.trie.is_dirty() {
            self.trie.save(&self.path).expect("Failed to save trie")
        }
    }
}

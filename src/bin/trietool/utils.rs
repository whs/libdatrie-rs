use iconv::{Iconv, IconvError};
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use datrie::alpha_map::AlphaMap;
use datrie::trie::Trie;
use datrie::types::AlphaChar;
use datrie::{CTrieData, TrieDeserializable, TrieSerializable};

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

    AutoSaveTrie::new(trie_path, alpha_map).expect("Failed to open file")
}

pub fn load_abm<R: Read>(alpha_map: &mut AlphaMap, stream: &mut BufReader<R>) -> io::Result<()> {
    // format: [begin_char,end_char]
    // where begin_char and end_char is in hex values
    let pattern = Regex::new(r"\s*\[\s*(?:0x)*([0-9a-f]+)\s*,\s*(?:0x)*([0-9a-f]+)\s*]").unwrap();

    for line in stream.lines() {
        if let Some(captures) = pattern.captures(&line.unwrap()) {
            let begin = captures.get(1).unwrap().as_str();
            let end = captures.get(2).unwrap().as_str();

            let Ok(begin_ac) = AlphaChar::from_str_radix(begin, 16) else {
                eprintln!("Failed to parse begin {}", begin);
                continue;
            };
            let Ok(end_ac) = AlphaChar::from_str_radix(end, 16) else {
                eprintln!("Failed to parse end {}", end);
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

pub struct AutoSaveTrie<T: Default + Clone + TrieSerializable + TrieDeserializable> {
    path: PathBuf,
    trie: Trie<T>,
    file: File,
}

impl<T: Default + Clone + TrieSerializable + TrieDeserializable> AutoSaveTrie<T> {
    pub fn new<P: AsRef<Path>>(path: P, alpha_map: AlphaMap) -> io::Result<Self> {
        let file = OpenOptions::new().write(true).create(true).open(&path)?;
        Ok(AutoSaveTrie {
            path: path.as_ref().to_path_buf(),
            trie: Trie::new(alpha_map),
            file: file,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut fp = BufReader::new(OpenOptions::new().read(true).write(true).open(&path)?);
        let trie = Trie::from_reader(&mut fp)?;
        Ok(AutoSaveTrie {
            path: path.as_ref().to_path_buf(),
            trie,
            file: fp.into_inner(),
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
            self.file.set_len(0).expect("Failed to truncate");
            self.file.seek(SeekFrom::Start(0)).expect("Failed to seek");
            self.trie
                .serialize(&mut self.file)
                .expect("Failed to save trie");
        }
    }
}

pub trait IconvExt {
    fn decode(&mut self, input: &[u8]) -> Result<Vec<u8>, IconvError>;
}

impl IconvExt for Iconv {
    fn decode(&mut self, input: &[u8]) -> Result<Vec<u8>, IconvError> {
        let mut buf = vec![0; input.len()];
        let mut read = 0;
        let mut written = 0;
        self.reset();
        loop {
            match self.convert(&input[read..], &mut buf[written..]) {
                Ok((r, w, _)) => {
                    read += r;
                    written += w;
                    if read >= input.len() {
                        buf.truncate(written);
                        return Ok(buf);
                    }
                    debug_assert!(!(r == 0 && w == 0));
                }
                Err((r, w, IconvError::NotSufficientOutput)) => {
                    read += r;
                    written += w;
                    buf.resize(buf.len() + w, 0);
                }
                Err((_, _, e)) => return Err(e),
            }
        }
    }
}

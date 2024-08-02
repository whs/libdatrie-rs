use crate::Cli;
use datrie::alpha_map::AlphaMap;
use datrie::trie::Trie;
use datrie::types::AlphaChar;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub fn load_trie(cli: &Cli) -> Trie {
    let trie_path = cli.path.join(format!("{}.tri", cli.trie));

    if let Ok(trie) = Trie::from_file(trie_path) {
        return trie;
    }

    let abm_path = cli.path.join(format!("{}.abm", cli.trie));
    let mut reader = BufReader::new(File::open(abm_path).expect("Cannot open alphabet map file"));
    let alpha_map = load_abm(&mut reader).expect("Fail to load alphabet map");

    Trie::new(alpha_map)
}

pub fn load_abm<R: Read>(stream: &mut BufReader<R>) -> io::Result<AlphaMap> {
    // format: [begin_char,end_char]
    // where begin_char and end_char is in hex values
    let mut alpha_map = AlphaMap::default();

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

    Ok(alpha_map)
}

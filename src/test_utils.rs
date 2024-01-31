use crate::AlphaMap;
use crate::Trie;

pub(crate) fn alphamap_new() -> AlphaMap {
    let mut out = AlphaMap::new();
    out.add_range(0x0061, 0x007a);

    out
}

pub(crate) fn trie_new() -> Trie {
    let alphamap = alphamap_new();
    Trie::new(alphamap)
}

pub(crate) const DICT_SRC: [&'static str; 39] = [
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

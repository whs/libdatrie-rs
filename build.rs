extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("TRIE_H")
        .with_include_version(true)
        .with_cpp_compat(true)
        .with_no_includes()
        .with_include("limits.h")
        .with_include("stdio.h")
        .rename_item("TrieIter", "TrieIterator")
        .with_after_include(
            r"
typedef enum { DA_FALSE = 0, DA_TRUE = 1 } Bool;
#define ALPHA_CHAR_ERROR   (~(AlphaChar)0)

#define TRIE_CHAR_TERM    '\0'
#define TRIE_CHAR_MAX     255

typedef int32 TrieIndex;
#define TRIE_INDEX_ERROR  0
#define TRIE_INDEX_MAX    0x7fffffff

#define   trie_state_is_terminal(s) trie_state_is_walkable((s),0)
#define   trie_state_is_leaf(s) (trie_state_is_single(s) && trie_state_is_terminal(s))
",
        )
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("trie.h");
}

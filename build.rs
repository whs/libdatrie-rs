extern crate cbindgen;
#[cfg(feature = "ctest")]
extern crate cc;

use std::env;

fn cbindgen_generate() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo::rustc-cdylib-link-arg=-fuse-ld=lld");
    println!(
        "cargo::rustc-cdylib-link-arg=-Wl,--version-script={}/libdatrie.map",
        crate_dir
    );

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("TRIE_H")
        .with_include_version(true)
        .with_cpp_compat(true)
        .with_no_includes()
        .rename_item("CTrie", "Trie")
        .rename_item("CTrieState", "TrieState")
        .rename_item("CTrieIterator", "TrieIterator")
        .rename_item("CTrieData", "TrieData")
        .with_include("stdint.h")
        .with_include("stdbool.h")
        .with_include("stdio.h")
        .with_after_include(
            r"
typedef enum { DA_FALSE = 0, DA_TRUE = 1 } Bool;
#ifndef FALSE
# define FALSE DA_FALSE
#endif
#ifndef TRUE
# define TRUE DA_TRUE
#endif
#define ALPHA_CHAR_ERROR   (~(AlphaChar)0)

#define TRIE_CHAR_MAX     255

#define   trie_state_is_terminal(s) trie_state_is_walkable((s),0)
#define   trie_state_is_leaf(s) (trie_state_is_single(s) && trie_state_is_terminal(s))
",
        )
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("trie.h");
}

fn ctest_generate() {
    #[cfg(feature = "ctest")]
    {
        cc::Build::new()
            .include(".")
            .file("src/ctest/utils.c")
            .file("src/ctest/test_byte_alpha.c")
            .file("src/ctest/test_byte_list.c")
            .file("src/ctest/test_file.c")
            .file("src/ctest/test_iterator.c")
            .file("src/ctest/test_nonalpha.c")
            .file("src/ctest/test_null_trie.c")
            .file("src/ctest/test_serialization.c")
            .file("src/ctest/test_store-retrieve.c")
            .file("src/ctest/test_term_state.c")
            .file("src/ctest/test_walk.c")
            .compile("ctest");
    }
}

fn main() {
    if env::var("CARGO_FEATURE_CFFI").is_ok() {
        cbindgen_generate();
    }

    // https://github.com/rust-lang/cargo/issues/4789
    if env::var("CARGO_FEATURE_CTEST").is_ok() {
        ctest_generate();
    }
}

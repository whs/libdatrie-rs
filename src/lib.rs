#![cfg_attr(not(feature = "std"), no_std)]
//#![feature(proc_macro_hygiene)]

extern crate alloc;

pub use types::{
    AlphaChar, AlphaCharToString, AsAlphaChar, TrieChar, TrieDeserializable, TrieIndex,
    TrieSerializable, ALPHA_CHAR_ERROR, TRIE_CHAR_MAX, TRIE_CHAR_TERM, TRIE_INDEX_ERROR,
    TRIE_INDEX_MAX,
};

pub use alpha_map::{AlphaMap, ToAlphaChars, ToTrieChar};

pub use trie::{Trie, TrieIterator, TrieState};

#[cfg(feature = "cffi")]
pub use types_c::CTrieData;
#[cfg(feature = "cffi")]
pub use types_c::TRIE_DATA_ERROR;

#[cfg_attr(not(feature = "cffi"), deny(unsafe_code))]
pub mod alpha_map;
mod darray;
#[cfg(feature = "cffi")]
mod fileutils;
mod symbols;
mod tail;
pub mod trie;
pub mod types;
#[cfg(feature = "cffi")]
mod types_c;

#[cfg(all(test, feature = "ctest"))]
mod ctest;
#[cfg(all(test, feature = "std"))]
mod testutils;
#[cfg(all(test, feature = "std"))]
mod trie_iter_test;
#[cfg(all(test, feature = "std"))]
mod trie_test;

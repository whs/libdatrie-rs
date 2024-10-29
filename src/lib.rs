#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use types::{
    AlphaChar, AlphaCharToString, AsAlphaChar, TrieChar, TrieIndex, ALPHA_CHAR_ERROR,
    TRIE_CHAR_MAX, TRIE_CHAR_TERM, TRIE_INDEX_ERROR, TRIE_INDEX_MAX,
};
#[cfg(feature = "std")]
pub use types::{TrieDeserializable, TrieSerializable};

pub use alpha_map::{AlphaMap, ToAlphaChars, ToTrieChar};

pub use trie::{ROTrie, Trie, TrieIterator, TrieState};

pub use types_c::CTrieData;
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
mod types_c;

#[cfg(all(test, feature = "ctest"))]
mod ctest;
#[cfg(all(test, feature = "std"))]
mod testutils;
#[cfg(all(test, feature = "std"))]
mod trie_iter_test;
#[cfg(all(test, feature = "std"))]
mod trie_test;

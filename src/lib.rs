#![feature(proc_macro_hygiene)]

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
#[cfg(test)]
mod testutils;
#[cfg(test)]
mod trie_iter_test;
#[cfg(test)]
mod trie_test;

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]

pub mod alpha_map;
mod darray;
mod dstring;
mod fileutils;
mod symbols;
mod tail;
pub mod trie;
mod trie_string;
pub mod types;

pub mod tests {
    pub mod test_byte_alpha;
    pub mod test_byte_list;
    pub mod test_file;
    pub mod test_iterator;
    pub mod test_nonalpha;
    pub mod test_null_trie;
    pub mod test_serialization;
    pub mod test_store_retrieve;
    pub mod test_term_state;
    pub mod test_walk;
    pub mod utils;
} // mod tests
pub mod tools {
    pub mod trietool;
} // mod tools

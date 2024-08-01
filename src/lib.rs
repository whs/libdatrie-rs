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
// pub mod tests {
//     pub mod test_byte_alpha;
//     pub mod test_byte_list;
//     pub mod test_file;
//     pub mod test_iterator;
//     pub mod test_nonalpha;
//     pub mod test_null_trie;
//     pub mod test_serialization;
//     pub mod test_store_retrieve;
//     pub mod test_term_state;
//     pub mod test_walk;
//     pub mod utils;
// } // mod tests
// pub mod tools {
//     pub mod trietool;
// } // mod tools

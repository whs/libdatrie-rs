use std::cmp::Ordering;
use std::slice;

#[cfg(feature = "cffi")]
use null_terminated::Nul;

pub type TrieIndex = i32;
pub const TRIE_INDEX_MAX: TrieIndex = 0x7fffffff;
pub const TRIE_INDEX_ERROR: TrieIndex = 0;

pub type AlphaChar = u32;
pub const ALPHA_CHAR_ERROR: AlphaChar = AlphaChar::MAX;

#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn alpha_char_strlen(str: *const AlphaChar) -> i32 {
    unsafe { Nul::new_unchecked(str) }.len() as i32
}

#[cfg(feature = "cffi")]
/// Return an AlphaChar string as slice, including the null byte
pub(crate) fn alpha_char_as_slice(str: *const AlphaChar) -> &'static [AlphaChar] {
    let len = alpha_char_strlen(str) as usize + 1;
    unsafe { slice::from_raw_parts(str, len) }
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn alpha_char_strcmp(str1: *const AlphaChar, str2: *const AlphaChar) -> i32 {
    let str1 = unsafe { Nul::new_unchecked(str1) };
    let str2 = unsafe { Nul::new_unchecked(str2) };
    match str1.cmp(str2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub type TrieChar = u8;
pub const TRIE_CHAR_TERM: TrieChar = '\0' as TrieChar;
pub const TRIE_CHAR_MAX: TrieChar = TrieChar::MAX;

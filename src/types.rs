use std::cmp::Ordering;
use null_terminated::Nul;

pub type Bool = u32;

pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub const FALSE: Bool = DA_FALSE;
pub const TRUE: Bool = DA_TRUE;

pub type TrieIndex = i32;
pub const TRIE_INDEX_MAX: TrieIndex = 0x7fffffff;

pub type AlphaChar = u32;
pub const ALPHA_CHAR_ERROR: AlphaChar = AlphaChar::MAX;

#[no_mangle]
pub extern "C" fn alpha_char_strlen(str: *const AlphaChar) -> i32 {
    unsafe { Nul::new_unchecked(str) }.len() as i32
}

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
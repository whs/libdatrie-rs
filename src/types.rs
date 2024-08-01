use std::cmp::Ordering;
use std::slice;

use null_terminated::Nul;

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Bool(u32);

pub(crate) const DA_TRUE: Bool = Bool(1);
pub(crate) const DA_FALSE: Bool = Bool(0);
pub(crate) const FALSE: Bool = DA_FALSE;
pub(crate) const TRUE: Bool = DA_TRUE;

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        match value {
            true => TRUE,
            false => FALSE,
        }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        match self.0 {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }
}

impl Into<u32> for Bool {
    fn into(self) -> u32 {
        self.0
    }
}

pub type TrieIndex = i32;
pub const TRIE_INDEX_MAX: TrieIndex = 0x7fffffff;
pub const TRIE_INDEX_ERROR: TrieIndex = 0;

pub type AlphaChar = u32;
pub const ALPHA_CHAR_ERROR: AlphaChar = AlphaChar::MAX;

#[no_mangle]
pub extern "C" fn alpha_char_strlen(str: *const AlphaChar) -> i32 {
    unsafe { Nul::new_unchecked(str) }.len() as i32
}

/// Return an AlphaChar string as slice, including the null byte
pub(crate) fn alpha_char_as_slice(str: *const AlphaChar) -> &'static [AlphaChar] {
    let len = alpha_char_strlen(str) as usize + 1;
    unsafe { slice::from_raw_parts(str, len) }
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

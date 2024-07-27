use std::ffi::CStr;
use std::slice;

use ::libc;
use null_terminated::Nul;

use crate::dstring::*;
use crate::types::*;

pub type TrieChar = u8;
pub const TRIE_CHAR_TERM: TrieChar = '\0' as TrieChar;

#[derive(Clone)]
#[repr(C)]
pub struct TrieString {
    ds: DString,
}

#[no_mangle]
pub extern "C" fn trie_char_strlen(key: *const TrieChar) -> usize {
    unsafe { Nul::new_unchecked(key) }.len()
}

#[no_mangle]
pub extern "C" fn trie_char_strsize(str: *const TrieChar) -> usize {
    trie_char_strlen(str) * size_of::<TrieChar>()
}

// trie_char_clone copies the C-string str into a Rust heap-allocated array.
// The array has the same length of the str in C plus the TRIE_CHAR_TERM byte.
pub(crate) fn trie_char_clone(str: *const TrieChar) -> Box<[TrieChar]> {
    let len = trie_char_strlen(str) + 1;

    let str_slice = unsafe { slice::from_raw_parts(str, len) };
    let cloned = Vec::from(str_slice);

    cloned.into_boxed_slice()
}

#[no_mangle]
pub extern "C" fn trie_char_strdup(str: *const TrieChar) -> *mut TrieChar {
    let len = trie_char_strlen(str) + 1;

    let str_slice = unsafe { slice::from_raw_parts(str, len) };
    let cloned = Vec::from(str_slice);

    Box::into_raw(cloned.into_boxed_slice()).cast()
}

#[no_mangle]
pub extern "C" fn trie_string_new(n_elm: libc::c_int) -> *mut TrieString {
    dstring_new(size_of::<TrieChar>() as i32, n_elm).cast()
}
#[no_mangle]
pub unsafe extern "C" fn trie_string_free(ts: *mut TrieString) {
    dstring_free(ts.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_length(ts: *const TrieString) -> libc::c_int {
    dstring_length(ts.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_get_val(ts: *const TrieString) -> *const libc::c_void {
    dstring_get_val(ts.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_get_val_rw(ts: *mut TrieString) -> *mut libc::c_void {
    dstring_get_val_rw(ts.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_clear(ts: *mut TrieString) {
    dstring_clear(ts.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_copy(dst: *mut TrieString, src: *const TrieString) -> Bool {
    dstring_copy(dst.cast(), src.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_append(dst: *mut TrieString, src: *const TrieString) -> Bool {
    dstring_append(dst.cast(), src.cast())
}
#[no_mangle]
pub extern "C" fn trie_string_append_string(ts: *mut TrieString, str: *const TrieChar) -> Bool {
    // In the C version this use strlen()
    let len = (unsafe { CStr::from_ptr(str.cast()) }).count_bytes();
    dstring_append_string(ts.cast(), str.cast(), len as i32)
}
#[no_mangle]
pub extern "C" fn trie_string_append_char(ts: *mut TrieString, tc: TrieChar) -> Bool {
    dstring_append_char(ts.cast(), (&tc as *const TrieChar).cast())
}
#[no_mangle]
pub extern "C" fn trie_string_terminate(ts: *mut TrieString) -> Bool {
    dstring_append_char(ts.cast(), (&TRIE_CHAR_TERM as *const TrieChar).cast())
}
#[no_mangle]
pub extern "C" fn trie_string_cut_last(ts: *mut TrieString) -> Bool {
    dstring_cut_last(ts.cast())
}

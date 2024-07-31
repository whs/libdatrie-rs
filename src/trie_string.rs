use std::ffi::CStr;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{io, slice};

use ::libc;

use crate::types::*;

pub type TrieChar = u8;
pub const TRIE_CHAR_TERM: TrieChar = '\0' as TrieChar;
pub const TRIE_CHAR_MAX: TrieChar = TrieChar::MAX;

pub(crate) fn trie_char_strlen(key: *const TrieChar) -> usize {
    unsafe { CStr::from_ptr(key.cast()) }.count_bytes()
}

/// Return a TrieChar string as slice, including the null byte
pub(crate) fn trie_char_as_slice<'a>(str: *const TrieChar) -> &'a [TrieChar] {
    let len = trie_char_strlen(str) + 1;

    unsafe { slice::from_raw_parts(str, len) }
}

/// trie_char_clone copies the C-string str into a Rust heap-allocated array.
/// The array has the same length of the str in C plus the TRIE_CHAR_TERM byte.
pub(crate) fn trie_char_clone(str: *const TrieChar) -> Box<[TrieChar]> {
    let str_slice = trie_char_as_slice(str);
    let cloned = Vec::from(str_slice);

    cloned.into_boxed_slice()
}

#[derive(Clone, Default, Debug)]
pub(crate) struct TrieString {
    inner: Vec<TrieChar>,
    str_len: usize,
}

impl TrieString {
    pub(crate) fn length(&self) -> usize {
        self.str_len
    }

    pub(crate) fn clear(&mut self) {
        self.str_len = 0;
        self.inner.clear();
    }

    pub(crate) fn append(&mut self, c: TrieChar) {
        self.inner.truncate(self.str_len);
        self.inner.push(c);
        self.str_len += 1;
    }

    pub(crate) fn append_from_str(&mut self, other: &TrieString) {
        self.inner.truncate(self.str_len);
        self.inner.extend_from_slice(&other);
        self.str_len += other.str_len;
    }

    pub(crate) fn append_from_slice(&mut self, other: &[u8]) {
        self.inner.truncate(self.str_len);
        self.inner.extend_from_slice(other);
        self.str_len += other.len();
    }

    pub(crate) fn ensure_terminated(&mut self) {
        self.inner.resize(self.str_len + 1, 0);
        self.inner[self.str_len] = 0;
        // This could desynchronize str_len, but I think it is unused?
    }

    pub(crate) fn pop(&mut self) -> Option<TrieChar> {
        let out = self.inner.pop();
        if out.is_some() {
            self.str_len -= 1;
        }

        out
    }
}

impl Deref for TrieString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner[..self.str_len]
    }
}

impl DerefMut for TrieString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner[..self.str_len]
    }
}

impl Write for TrieString {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.append_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[deprecated(note = "Use TrieString::default(). Note that it doesn't preallocate!")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_new(n_elm: i32) -> *mut TrieString {
    let ts = TrieString {
        inner: Vec::with_capacity(n_elm as usize),
        str_len: 0,
    };
    Box::into_raw(Box::new(ts))
}

#[no_mangle]
pub(crate) unsafe extern "C" fn trie_string_free(ts: *mut TrieString) {
    drop(Box::from_raw(ts))
}

#[deprecated(note = "Use ts.length()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_length(ts: *const TrieString) -> i32 {
    let str = unsafe { &*ts };
    str.length() as i32
}

#[deprecated(note = "Use ts.deref()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_get_val(ts: *const TrieString) -> *const libc::c_void {
    let str = unsafe { &*ts };
    str.deref().as_ptr().cast()
}

#[deprecated(note = "Use ts.deref_mut()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_get_val_rw(mut ts: NonNull<TrieString>) -> *mut libc::c_void {
    let str = unsafe { ts.as_mut() };
    str.deref_mut().as_mut_ptr().cast()
}

#[deprecated(note = "Use ts.clear()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_clear(mut ts: NonNull<TrieString>) {
    let str = unsafe { ts.as_mut() };
    str.clear()
}

#[deprecated(note = "Use src.clone()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_copy(mut dst: NonNull<TrieString>, src: *const TrieString) -> Bool {
    let src = unsafe { &*src };
    let dst = unsafe { dst.as_mut() };

    dst.clone_from(src);

    TRUE
}

#[deprecated(note = "Use dst.append_from_str()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_append(mut dst: NonNull<TrieString>, src: *const TrieString) -> Bool {
    let src = unsafe { &*src };
    let dst = unsafe { dst.as_mut() };
    dst.append_from_str(src);
    TRUE
}

#[deprecated(note = "Use dst.append_from_slice()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_append_string(
    mut ts: NonNull<TrieString>,
    str: *const TrieChar,
) -> Bool {
    // XXX: The C version doesn't copy the null terminator
    let len = trie_char_strlen(str);
    // XXX: Using CStr here is not correct, as TrieChar is u8 and CStr is i8
    let str_slice = unsafe { slice::from_raw_parts(str, len) };

    let ts = unsafe { ts.as_mut() };
    ts.append_from_slice(str_slice);
    TRUE
}

#[deprecated(note = "Use ts.append()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_append_char(mut ts: NonNull<TrieString>, tc: TrieChar) -> Bool {
    let ts = unsafe { ts.as_mut() };
    ts.append(tc);
    TRUE
}

#[deprecated(note = "Use ts.ensure_terminated()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_terminate(mut ts: NonNull<TrieString>) -> Bool {
    let ts = unsafe { ts.as_mut() };
    ts.ensure_terminated();
    TRUE
}

#[deprecated(note = "Use ts.pop()")]
#[no_mangle]
pub(crate) extern "C" fn trie_string_cut_last(mut ts: NonNull<TrieString>) -> Bool {
    let ts = unsafe { ts.as_mut() };
    ts.pop().is_some().into()
}

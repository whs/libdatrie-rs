use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::Cell;
use core::ops::Deref;
use core::ptr::NonNull;
use core::{cmp, iter, ptr, slice};
use std::ffi::{CStr, OsStr};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
#[cfg(unix)]
use std::os::unix::prelude::OsStrExt;
use std::path::Path;

#[cfg(feature = "cffi")]
use ::libc;

use crate::alpha_map::{AlphaMap, ToAlphaChars};
use crate::darray::DArray;
#[cfg(feature = "cffi")]
use crate::fileutils::wrap_cfile_nonnull;
use crate::tail::Tail;
use crate::types::TRIE_CHAR_TERM;
use crate::types::*;
#[cfg(feature = "cffi")]
use crate::types_c::{Bool, CTrieData, FALSE, TRIE_DATA_ERROR, TRUE};

pub struct Trie<TrieData: Default> {
    alpha_map: AlphaMap,
    da: DArray,
    tail: Tail<TrieData>,
    is_dirty: Cell<bool>,
}

impl<TrieData: Default> Trie<TrieData> {
    /// Create a new empty trie object based on the given `alpha_map` alphabet
    /// set. The trie contents can then be added and deleted with trie.store() and
    /// trie.delete() respectively.
    pub fn new(alpha_map: AlphaMap) -> Self {
        Self {
            alpha_map,
            da: DArray::default(),
            tail: Tail::default(),
            is_dirty: Cell::new(true),
        }
    }

    /// Check if the trie is dirty with some pending changes and needs saving
    /// to keep the file synchronized.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty.get()
    }

    pub fn store(&mut self, key: &[AlphaChar], data: TrieData) -> bool {
        self.store_conditionally(key, data, true)
    }

    pub fn store_if_absent(&mut self, key: &[AlphaChar], data: TrieData) -> bool {
        self.store_conditionally(key, data, false)
    }

    fn store_conditionally(
        &mut self,
        key: &[AlphaChar],
        data: TrieData,
        is_overwrite: bool,
    ) -> bool {
        // walk through branches
        let mut s = self.da.get_root();
        let mut p = key;
        while !self.da.is_separate(s) {
            let Some(tc) = self.alpha_map.char_to_trie(p[0]) else {
                return false;
            };
            if let Some(next_s) = self.da.walk(s, tc as TrieChar) {
                s = next_s;
            } else {
                let Some(key_str) = self.alpha_map.char_to_trie_str(p) else {
                    return false;
                };
                return self.branch_in_branch(s, &key_str, data).into();
            }
            if p[0] == 0 {
                break;
            }
            p = &p[1..];
        }

        // walk through tail
        let sep = p;
        let t = self.da.get_tail_index(s);
        let mut suffix_idx = 0;
        for ch in p.iter().copied() {
            let Some(tc) = self.alpha_map.char_to_trie(ch) else {
                return false;
            };
            if let Some(next_idx) = self.tail.walk_char(t, suffix_idx, tc as TrieChar) {
                suffix_idx = next_idx;
            } else {
                let Some(tail_str) = self.alpha_map.char_to_trie_str(sep) else {
                    return false;
                };
                return self.branch_in_tail(s, &tail_str, data).into();
            }
            if ch == 0 {
                break;
            }
        }

        // duplicated, overwrite val if flagged
        if !is_overwrite {
            return false;
        }
        self.tail.set_data(t, data);
        self.is_dirty.set(true);
        true
    }

    pub fn root(&self) -> TrieState<TrieData> {
        TrieState::new(self, self.da.get_root(), 0, false)
    }

    pub fn retrieve(&self, key: &[AlphaChar]) -> Option<&TrieData> {
        // walk through branches
        let mut s = self.da.get_root();
        let mut key_iter = key.iter().copied();
        let mut last_ch = ALPHA_CHAR_ERROR;
        while let Some(ch) = key_iter.next() {
            last_ch = ch;
            if self.da.is_separate(s) {
                break;
            }
            let tc = self.alpha_map.char_to_trie(ch)?;
            s = self.da.walk(s, tc as TrieChar)?;
            if ch == 0 {
                break;
            }
        }

        // walk through tail
        s = self.da.get_tail_index(s);
        let mut suffix_idx = 0;
        // start iterating from the last character
        for ch in iter::once(last_ch).chain(key_iter) {
            let tc = self.alpha_map.char_to_trie(ch)?;
            suffix_idx = self.tail.walk_char(s, suffix_idx, tc as TrieChar)?;
        }

        // found
        // unwrap as an assertion since this should never fail
        Some(self.tail.get_data(s).unwrap())
    }

    fn branch_in_branch(
        &mut self,
        sep_node: TrieIndex,
        suffix: &[TrieChar],
        data: TrieData,
    ) -> bool {
        let mut suffix = suffix;
        let Some(new_da) = self.da.insert_branch(sep_node, suffix[0]) else {
            return false;
        };
        if suffix[0] != TRIE_CHAR_TERM {
            suffix = &suffix[1..];
        }

        let new_tail = self.tail.add_suffix(Some(suffix.into()));
        self.tail.set_data(new_tail, data);
        self.da.set_tail_index(new_da, new_tail);

        self.is_dirty.set(true);
        true
    }

    fn branch_in_tail(&mut self, sep_node: TrieIndex, suffix: &[TrieChar], data: TrieData) -> bool {
        // adjust separate point in old path
        let old_tail = self.da.get_tail_index(sep_node);
        let Some(old_suffix) = self.tail.get_suffix(old_tail) else {
            return false;
        };

        let mut p = old_suffix;
        let mut s = sep_node;
        let mut suffix = suffix;
        while p[0] == suffix[0] {
            let Some(t) = self.da.insert_branch(s, p[0]) else {
                // TODO: Move to fail() code
                self.da.prune_upto(sep_node, s);
                self.da.set_tail_index(sep_node, old_tail);
                return false;
            };
            s = t;

            p = &p[1..];
            suffix = &suffix[1..];
        }

        let Some(old_da) = self.da.insert_branch(s, p[0]) else {
            // TODO: Move to fail() code
            self.da.prune_upto(sep_node, s);
            self.da.set_tail_index(sep_node, old_tail);
            return false;
        };

        if p[0] != TRIE_CHAR_TERM {
            p = &p[1..];
        }
        self.tail.set_suffix(old_tail, Some(p.into()));
        self.da.set_tail_index(old_da, old_tail);

        // insert the new branch at the new separate point
        self.branch_in_branch(s, suffix, data)
    }

    pub fn delete(&mut self, key: &[AlphaChar]) -> bool {
        let mut s = self.da.get_root();
        let mut p = key;
        while !self.da.is_separate(s) {
            let Some(tc) = self.alpha_map.char_to_trie(p[0]) else {
                return false;
            };
            if let Some(new_s) = self.da.walk(s, tc as TrieChar) {
                s = new_s;
            } else {
                return false;
            }
            if p[0] == 0 {
                break;
            }
            p = &p[1..];
        }

        let t = self.da.get_tail_index(s);
        let mut suffix_idx = 0;

        for ch in p.iter().copied() {
            let Some(tc) = self.alpha_map.char_to_trie(ch) else {
                return false;
            };
            if let Some(new_idx) = self.tail.walk_char(t, suffix_idx, tc as TrieChar) {
                suffix_idx = new_idx;
            } else {
                return false;
            }
            if ch == 0 {
                break;
            }
        }

        self.tail.delete(t);
        self.da.set_base(s, TRIE_INDEX_ERROR);
        self.da.prune(s);

        self.is_dirty.set(true);
        true
    }
}

impl<TrieData: TrieSerializable + Default> Trie<TrieData> {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut fp = BufWriter::new(File::create(path)?);
        self.serialize(&mut fp)
    }

    pub fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        self.alpha_map.serialize(writer)?;
        self.da.serialize(writer)?;
        self.tail.serialize(writer)?;
        self.is_dirty.set(false);
        Ok(())
    }

    /// Returns size that would be occupied by a trie if it was
    /// serialized into a binary blob or file.
    pub fn serialized_size(&self) -> usize {
        self.alpha_map.serialized_size() + self.da.serialized_size() + self.tail.serialized_size()
    }
}

impl<TrieData: TrieDeserializable + Default + Clone> Trie<TrieData> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut fp = BufReader::new(File::open(path)?);
        Self::from_reader(&mut fp)
    }

    /// Create a new trie and initialize its contents by reading from a reader.
    /// This function guaranteed that only the trie has been read from the reader.
    /// This can be useful for embedding trie index as part of file data.
    pub fn from_reader<T: Read>(reader: &mut T) -> io::Result<Self> {
        let alpha_map = AlphaMap::read(reader)?;
        let da = DArray::read(reader)?;
        let tail = Tail::read(reader)?;

        Ok(Self {
            alpha_map,
            da,
            tail,
            is_dirty: Cell::new(false),
        })
    }
}

impl<TrieData: Clone + Default> Trie<TrieData> {
    pub fn iter(&self) -> TrieIterator<TrieData> {
        TrieIterator::new_from_trie(self)
    }
}

#[cfg(feature = "cffi")]
pub(crate) type CTrie = Trie<Option<CTrieData>>;

#[deprecated(note = "Use Trie::new()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_new(alpha_map: *const AlphaMap) -> *mut CTrie {
    let trie = Trie::new(unsafe { &*alpha_map }.clone());
    Box::into_raw(Box::new(trie))
}

#[deprecated(note = "Use Trie::from_file()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_new_from_file(path: *const libc::c_char) -> *mut CTrie {
    let str = unsafe { CStr::from_ptr(path) };
    let osstr = OsStr::from_bytes(str.to_bytes());
    let Ok(trie) = Trie::from_file(osstr) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(trie))
}

#[deprecated(note = "Use Trie::from_reader()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_fread(file: NonNull<libc::FILE>) -> *mut CTrie {
    let mut file = wrap_cfile_nonnull(file);
    let Ok(trie) = Trie::from_reader(&mut file) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(trie))
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub unsafe extern "C" fn trie_free(trie: *mut CTrie) {
    drop(Box::from_raw(trie))
}

#[deprecated(note = "Use trie.save()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_save(mut trie: NonNull<CTrie>, path: *const libc::c_char) -> i32 {
    let trie = unsafe { trie.as_mut() };
    let str = unsafe { CStr::from_ptr(path) };
    let osstr = OsStr::from_bytes(str.to_bytes());
    match trie.save(osstr) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[deprecated(note = "Use trie.serialized_size()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_get_serialized_size(trie: *const CTrie) -> libc::size_t {
    let trie = unsafe { &*trie };
    trie.serialized_size()
}

#[deprecated(note = "Use trie.serialize()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_serialize(mut trie: NonNull<CTrie>, ptr: *mut u8) {
    // Seems that this doesn't actually move the pointer?
    let trie = unsafe { trie.as_mut() };
    let slice = unsafe { slice::from_raw_parts_mut(ptr, trie.serialized_size()) };
    let mut cursor = Cursor::new(slice);
    trie.serialize(&mut cursor).unwrap();
}

#[deprecated(note = "Use trie.serialize()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_fwrite(mut trie: NonNull<CTrie>, file: NonNull<libc::FILE>) -> i32 {
    let trie = unsafe { trie.as_mut() };
    let mut file = wrap_cfile_nonnull(file);
    match trie.serialize(&mut file) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[deprecated(note = "Use trie.is_dirty()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_is_dirty(trie: *const CTrie) -> Bool {
    let trie = unsafe { &*trie };
    trie.is_dirty().into()
}

#[deprecated(note = "Use trie.retrieve()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_retrieve(
    trie: *const CTrie,
    key: *const AlphaChar,
    o_data: *mut CTrieData,
) -> Bool {
    let trie = unsafe { &*trie };
    let key_slice = alpha_char_as_slice(key);

    match trie.retrieve(key_slice).copied() {
        Some(v) => {
            if !o_data.is_null() {
                unsafe {
                    o_data.write(v.unwrap_or(TRIE_DATA_ERROR));
                }
            }
            TRUE
        }
        None => FALSE,
    }
}

#[deprecated(note = "Use trie.store()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_store(
    mut trie: NonNull<CTrie>,
    key: *const AlphaChar,
    data: CTrieData,
) -> Bool {
    let trie = unsafe { trie.as_mut() };
    let key_slice = alpha_char_as_slice(key);

    trie.store_conditionally(key_slice, Some(data), true).into()
}

#[deprecated(note = "Use trie.store_if_absent()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_store_if_absent(
    mut trie: NonNull<CTrie>,
    key: *const AlphaChar,
    data: CTrieData,
) -> Bool {
    let trie = unsafe { trie.as_mut() };
    let key_slice = alpha_char_as_slice(key);

    trie.store_conditionally(key_slice, Some(data), false)
        .into()
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_delete(mut trie: NonNull<CTrie>, key: *const AlphaChar) -> Bool {
    let trie = unsafe { trie.as_mut() };
    trie.delete(alpha_char_as_slice(key)).into()
}

#[cfg(feature = "cffi")]
pub type TrieEnumFunc =
    unsafe extern "C" fn(*const AlphaChar, CTrieData, *mut libc::c_void) -> Bool;

#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_enumerate(
    trie: *const CTrie,
    enum_func: TrieEnumFunc,
    user_data: *mut libc::c_void,
) -> Bool {
    let trie = unsafe { &*trie };

    let mut cont = true;
    for (key, data) in trie.iter() {
        cont = unsafe {
            enum_func(
                key.as_ptr(),
                data.flatten().unwrap_or(TRIE_DATA_ERROR),
                user_data,
            )
            .into()
        };
    }

    cont.into()
}

#[deprecated(note = "Use trie.root()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_root<'a>(trie: *const CTrie) -> *mut CTrieState<'a> {
    let trie = unsafe { &*trie };
    Box::into_raw(Box::new(trie.root()))
}

#[derive(Clone)]
pub struct TrieState<'a, TrieData: Default> {
    /// the corresponding trie
    trie: &'a Trie<TrieData>,
    /// index in double-array/tail structures
    index: TrieIndex,
    /// suffix character offset, if in suffix
    suffix_idx: i16,
    /// whether it is currently in suffix part
    is_suffix: bool,
}

impl<'a, TrieData: Default> TrieState<'a, TrieData> {
    fn new(
        trie: &Trie<TrieData>,
        index: TrieIndex,
        suffix_idx: i16,
        is_suffix: bool,
    ) -> TrieState<TrieData> {
        TrieState {
            trie,
            index,
            suffix_idx,
            is_suffix,
        }
    }

    pub fn rewind(&mut self) {
        self.index = self.trie.da.get_root();
        self.is_suffix = false;
    }

    pub fn walk(&mut self, c: AlphaChar) -> bool {
        let Some(tc) = self.trie.alpha_map.char_to_trie(c) else {
            return false;
        };
        if !self.is_suffix {
            if let Some(next_idx) = self.trie.da.walk(self.index, tc as TrieChar) {
                self.index = next_idx;
                if self.trie.da.is_separate(self.index) {
                    self.index = self.trie.da.get_tail_index(self.index);
                    self.suffix_idx = 0;
                    self.is_suffix = true;
                }
                return true;
            } else {
                return false;
            }
        } else {
            if let Some(next_idx) =
                self.trie
                    .tail
                    .walk_char(self.index, self.suffix_idx, tc as TrieChar)
            {
                self.suffix_idx = next_idx;
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn is_walkable(&self, c: AlphaChar) -> bool {
        let Some(tc) = self.trie.alpha_map.char_to_trie(c) else {
            return false;
        };
        if !self.is_suffix {
            self.trie.da.is_walkable(self.index, tc as TrieChar)
        } else {
            self.trie
                .tail
                .is_walkable_char(self.index, self.suffix_idx, tc as TrieChar)
        }
    }

    pub fn walkable_chars(&self) -> Vec<AlphaChar> {
        if !self.is_suffix {
            self.trie
                .da
                .output_symbols(self.index)
                .iter()
                .copied()
                .map_to_alpha_char(&self.trie.alpha_map)
                .collect()
        } else {
            let suffix = self.trie.tail.get_suffix(self.index).unwrap();
            vec![self
                .trie
                .alpha_map
                .trie_to_char(suffix[self.suffix_idx as usize])]
        }
    }

    pub fn is_single(&self) -> bool {
        self.is_suffix
    }

    pub fn is_terminal(&self) -> bool {
        self.is_walkable(0)
    }

    pub fn is_leaf(&self) -> bool {
        self.is_single() && self.is_terminal()
    }

    pub fn get_data(&self) -> Option<&TrieData> {
        if !self.is_suffix {
            if let Some(index) = self.trie.da.walk(self.index, TRIE_CHAR_TERM) {
                if self.trie.da.is_separate(index) {
                    let tail_index = self.trie.da.get_tail_index(index);
                    return self.trie.tail.get_data(tail_index);
                }
            }
        } else {
            if self
                .trie
                .tail
                .is_walkable_char(self.index, self.suffix_idx, TRIE_CHAR_TERM)
            {
                return self.trie.tail.get_data(self.index);
            }
        }

        None
    }
}

#[cfg(feature = "cffi")]
pub(crate) type CTrieState<'a> = TrieState<'a, Option<CTrieData>>;

#[deprecated(note = "Use TrieState.clone_from()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_copy<'a>(
    mut dst: NonNull<CTrieState<'a>>,
    src: *const CTrieState<'a>,
) {
    let dst = unsafe { dst.as_mut() };
    let src = unsafe { &*src };

    dst.clone_from(src);
}

#[deprecated(note = "Use TrieState.clone()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_clone(s: *const CTrieState) -> *mut CTrieState {
    let state = unsafe { &*s };
    let cloned = state.clone();
    Box::into_raw(Box::new(cloned))
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub unsafe extern "C" fn trie_state_free(s: NonNull<CTrieState>) {
    drop(Box::from_raw(s.as_ptr()))
}

#[deprecated(note = "Use s.rewind()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_rewind(mut s: NonNull<CTrieState>) {
    let state = unsafe { s.as_mut() };
    state.rewind();
}

#[deprecated(note = "Use s.walk()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_walk(mut s: NonNull<CTrieState>, c: AlphaChar) -> Bool {
    let state = unsafe { s.as_mut() };
    state.walk(c).into()
}

#[deprecated(note = "Use s.is_walkable()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_is_walkable(s: *const CTrieState, c: AlphaChar) -> Bool {
    let state = unsafe { &*s };
    state.is_walkable(c).into()
}

#[deprecated(note = "Use chars = s.walkable_chars()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_walkable_chars(
    s: *const CTrieState,
    chars: NonNull<AlphaChar>,
    chars_nelm: i32,
) -> i32 {
    let state = unsafe { &*s };
    let chars = unsafe { slice::from_raw_parts_mut(chars.as_ptr(), chars_nelm as usize) };

    let out = state.walkable_chars();

    let copy_len = cmp::min(out.len(), chars.len());
    chars[..copy_len].copy_from_slice(&out[..copy_len]);

    copy_len as i32
}

#[deprecated(note = "Use s.is_single()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_is_single(s: *const CTrieState) -> Bool {
    let state = unsafe { &*s };
    state.is_single().into()
}

#[deprecated(note = "Use s.get_data().unwrap_or(TRIE_DATA_ERROR)")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_state_get_data(s: *const CTrieState) -> CTrieData {
    let Some(state) = (unsafe { s.as_ref() }) else {
        return TRIE_DATA_ERROR;
    };
    state
        .get_data()
        .copied()
        .flatten()
        .unwrap_or(TRIE_DATA_ERROR)
}

pub struct TrieIterator<'trie: 'state, 'state, TrieData: Default + Clone> {
    root: Cow<'state, TrieState<'trie, TrieData>>,
    state: Option<TrieState<'trie, TrieData>>,
    key: Vec<TrieChar>,
}

impl<'trie: 'state, 'state, TrieData: Default + Clone> TrieIterator<'trie, 'state, TrieData> {
    pub fn new(root: &'state TrieState<'trie, TrieData>) -> TrieIterator<'trie, 'state, TrieData> {
        TrieIterator {
            root: Cow::Borrowed(root),
            state: None,
            key: Vec::<TrieChar>::default(),
        }
    }

    pub fn new_from_trie(trie: &'trie Trie<TrieData>) -> TrieIterator<'trie, 'state, TrieData> {
        TrieIterator {
            root: Cow::Owned(trie.root()),
            state: None,
            key: Vec::<TrieChar>::default(),
        }
    }

    pub fn key(&self) -> Option<Vec<AlphaChar>> {
        let state = self.state.as_ref()?;

        let mut tail_str;
        let mut out = Vec::new();

        // if state in tail, root == state
        if state.is_suffix {
            tail_str = state.trie.tail.get_suffix(state.index)?;
            tail_str = &tail_str[(state.suffix_idx as usize)..];
        } else {
            let tail_idx = state.trie.da.get_tail_index(state.index);
            tail_str = state.trie.tail.get_suffix(tail_idx)?;

            // Add current key to the output
            out.extend(
                self.key
                    .iter()
                    .copied()
                    .map_to_alpha_char(&state.trie.alpha_map),
            )
        }

        out.extend(
            tail_str
                .iter()
                .copied()
                .map_to_alpha_char(&state.trie.alpha_map),
        );
        out.push(0);

        Some(out)
    }

    pub fn data(&self) -> Option<&TrieData> {
        let state = self.state.as_ref()?;

        let tail_index;

        if !state.is_suffix {
            if !state.trie.da.is_separate(state.index) {
                return None;
            }
            tail_index = state.trie.da.get_tail_index(state.index);
        } else {
            tail_index = state.index;
        }

        state.trie.tail.get_data(tail_index)
    }

    fn iter_next(&mut self) -> bool {
        return match &mut self.state {
            Some(state) => {
                // no next entry for tail state
                if state.is_suffix {
                    return false;
                }

                let Some(sep) =
                    state
                        .trie
                        .da
                        .next_separate(self.root.index, state.index, &mut self.key)
                else {
                    return false;
                };
                state.index = sep;
                true
            }
            None => {
                let state = self.state.insert(self.root.deref().clone());

                // for tail state, we are already at the only entry
                if state.is_suffix {
                    return true;
                }

                let Some(sep) = state.trie.da.first_separate(state.index, &mut self.key) else {
                    return false;
                };
                state.index = sep;
                true
            }
        };
    }
}

impl<'trie: 'state, 'state, TrieData: Default + Clone> Iterator
    for TrieIterator<'trie, 'state, TrieData>
{
    type Item = (Vec<AlphaChar>, Option<TrieData>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_next() {
            true => Some((self.key().unwrap(), self.data().cloned())),
            false => None,
        }
    }
}

#[cfg(feature = "cffi")]
pub(crate) type CTrieIterator<'a, 'b> = TrieIterator<'a, 'b, Option<CTrieData>>;

#[deprecated(note = "Use TrieIterator::new()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_iterator_new(s: NonNull<CTrieState>) -> *mut CTrieIterator {
    let i = CTrieIterator::new(unsafe { s.as_ref() });
    Box::into_raw(Box::new(i))
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_free(iter: NonNull<CTrieIterator>) {
    drop(Box::from_raw(iter.as_ptr()))
}

#[deprecated(note = "Use iter as Iterator")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_iterator_next(mut iter: NonNull<CTrieIterator>) -> Bool {
    let iter = unsafe { iter.as_mut() };
    iter.iter_next().into()
}

#[deprecated(note = "Use iter.key()")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_iterator_get_key(iter: *const CTrieIterator) -> *mut AlphaChar {
    let iter = unsafe { &*iter };
    match iter.key() {
        Some(key) => Box::into_raw(key.into_boxed_slice()).cast(),
        None => ptr::null_mut(),
    }
}

#[deprecated(note = "Use iter.data().unwrap_or(TRIE_DATA_ERROR)")]
#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn trie_iterator_get_data(iter: *const CTrieIterator) -> CTrieData {
    let iter = unsafe { &*iter };
    iter.data().copied().flatten().unwrap_or(TRIE_DATA_ERROR)
}

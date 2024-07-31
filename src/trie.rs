use std::cell::Cell;
use std::ffi::{CStr, OsStr};
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::ptr::NonNull;
use std::{io, iter, ptr, slice};

use ::libc;

use crate::alpha_map::{
    alpha_map_char_to_trie, alpha_map_char_to_trie_str, alpha_map_trie_to_char, AlphaMap,
};
use crate::darray::{
    da_first_separate, da_get_base, da_get_check, da_get_root, da_insert_branch, da_next_separate,
    da_output_symbols, da_prune, da_prune_upto, da_set_base, da_walk, DArray,
};
use crate::fileutils::wrap_cfile_nonnull;
use crate::tail::{
    tail_delete, tail_get_data, tail_get_suffix, tail_set_data, tail_set_suffix, tail_walk_char,
    Tail,
};
use crate::trie_string::{
    trie_char_as_slice, trie_char_strlen, trie_string_free, trie_string_get_val,
    trie_string_length, trie_string_new, TrieString, TRIE_CHAR_TERM,
};
use crate::types::*;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;

pub type TrieChar = u8;

pub type TrieData = i32;
pub const TRIE_DATA_ERROR: TrieData = -1;

#[repr(C)]
pub struct Trie {
    alpha_map: AlphaMap,
    da: DArray,
    tail: Tail,
    is_dirty: Cell<bool>,
}

impl Trie {
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

    /// Check if the trie is dirty with some pending changes and needs saving
    /// to keep the file synchronized.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty.get()
    }

    // fn store_conditionally(&mut self, key: &[AlphaChar], data: TrieData, is_overwrite: bool) -> bool {
    //
    // }

    pub fn retrieve(&self, key: &[AlphaChar]) -> Option<TrieData> {
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
        self.tail.set_data(new_tail, Some(data));
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
}

pub type TrieEnumFunc =
    Option<unsafe extern "C" fn(*const AlphaChar, TrieData, *mut libc::c_void) -> Bool>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrieState {
    trie: *mut Trie,
    index: TrieIndex,
    suffix_idx: libc::c_short,
    is_suffix: libc::c_short,
}

#[repr(C)]
pub struct TrieIterator {
    root: *const TrieState,
    state: *mut TrieState,
    key: *mut TrieString,
}

pub const NULL: libc::c_int = 0 as libc::c_int;

#[deprecated(note = "Use Trie::new()")]
#[no_mangle]
pub extern "C" fn trie_new(alpha_map: *const AlphaMap) -> *mut Trie {
    println!("trie_new: Rust!");
    let trie = Trie::new(unsafe { &*alpha_map }.clone());
    Box::into_raw(Box::new(trie))
}

#[deprecated(note = "Use Trie::from_file()")]
#[no_mangle]
pub extern "C" fn trie_new_from_file(path: *const libc::c_char) -> *mut Trie {
    println!("trie_new_from_file: Rust!");
    let str = unsafe { CStr::from_ptr(path) };
    let osstr = OsStr::from_bytes(str.to_bytes());
    let Ok(trie) = Trie::from_file(osstr) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(trie))
}

#[deprecated(note = "Use Trie::from_reader()")]
#[no_mangle]
pub extern "C" fn trie_fread(file: NonNull<libc::FILE>) -> *mut Trie {
    let mut file = wrap_cfile_nonnull(file);
    let Ok(trie) = Trie::from_reader(&mut file) else {
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(trie))
}

#[no_mangle]
pub unsafe extern "C" fn trie_free(trie: *mut Trie) {
    drop(Box::from_raw(trie))
}

#[deprecated(note = "Use trie.save()")]
#[no_mangle]
pub extern "C" fn trie_save(mut trie: NonNull<Trie>, path: *const libc::c_char) -> i32 {
    let trie = unsafe { trie.as_mut() };
    let str = unsafe { CStr::from_ptr(path) };
    let osstr = OsStr::from_bytes(str.to_bytes());
    match trie.save(osstr) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[deprecated(note = "Use trie.serialized_size()")]
#[no_mangle]
pub extern "C" fn trie_get_serialized_size(trie: *const Trie) -> usize {
    let trie = unsafe { &*trie };
    trie.serialized_size()
}

#[deprecated(note = "Use trie.serialize()")]
#[no_mangle]
pub extern "C" fn trie_serialize(mut trie: NonNull<Trie>, ptr: *mut u8) {
    // Seems that this doesn't actually move the pointer?
    let trie = unsafe { trie.as_mut() };
    let slice = unsafe { slice::from_raw_parts_mut(ptr, trie.serialized_size()) };
    let mut cursor = Cursor::new(slice);
    trie.serialize(&mut cursor).unwrap();
}

#[deprecated(note = "Use trie.serialize()")]
#[no_mangle]
pub extern "C" fn trie_fwrite(mut trie: NonNull<Trie>, file: NonNull<libc::FILE>) -> i32 {
    let trie = unsafe { trie.as_mut() };
    let mut file = wrap_cfile_nonnull(file);
    match trie.serialize(&mut file) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[deprecated(note = "Use trie.is_dirty()")]
#[no_mangle]
pub extern "C" fn trie_is_dirty(trie: *const Trie) -> Bool {
    let trie = unsafe { &*trie };
    trie.is_dirty().into()
}

#[deprecated(note = "Use trie.retrieve()")]
#[no_mangle]
pub extern "C" fn trie_retrieve(
    trie: *const Trie,
    key: *const AlphaChar,
    o_data: *mut TrieData,
) -> Bool {
    let trie = unsafe { &*trie };
    let key_slice = alpha_char_as_slice(key);

    match trie.retrieve(key_slice) {
        Some(v) => {
            if !o_data.is_null() {
                unsafe {
                    o_data.write(v);
                }
            }
            TRUE
        }
        None => FALSE,
    }
}

#[no_mangle]
pub unsafe extern "C" fn trie_store(
    mut trie: *mut Trie,
    mut key: *const AlphaChar,
    mut data: TrieData,
) -> Bool {
    return trie_store_conditionally(trie, key, data, true);
}

#[no_mangle]
pub unsafe extern "C" fn trie_store_if_absent(
    mut trie: *mut Trie,
    mut key: *const AlphaChar,
    mut data: TrieData,
) -> Bool {
    return trie_store_conditionally(trie, key, data, false);
}

unsafe fn trie_store_conditionally(
    mut trie: *mut Trie,
    mut key: *const AlphaChar,
    data: TrieData,
    is_overwrite: bool,
) -> Bool {
    let trie = unsafe { &mut *trie };
    let key_slice = alpha_char_as_slice(key);

    // walk through branches
    let mut s = trie.da.get_root();
    let mut p = key;
    while !trie.da.is_separate(s) {
        let Some(mut tc) = trie.alpha_map.char_to_trie(*p) else {
            return FALSE;
        };
        if let Some(next_s) = trie.da.walk(s, tc as TrieChar) {
            s = next_s;
        } else {
            // TODO: alpha_char_as_slice is not needed if we use key_slice and not p
            let Some(key_str) = trie.alpha_map.char_to_trie_str(alpha_char_as_slice(p)) else {
                return FALSE;
            };
            return trie.branch_in_branch(s, &key_str, data).into();
        }
        if *p == 0 {
            break;
        }
        p = p.offset(1);
    }

    // walk through tail
    let sep = p;
    let t = -trie.da.get_tail_index(s);
    let mut suffix_idx = 0;
    loop {
        let Some(mut tc) = trie.alpha_map.char_to_trie(*p) else {
            return FALSE;
        };
        if let Some(next_idx) = trie.tail.walk_char(t, suffix_idx, tc as TrieChar) {
            suffix_idx = next_idx;
        } else {
            // TODO: alpha_char_as_slice is not needed if we use key_slice and not p
            let Some(tail_str) = trie.alpha_map.char_to_trie_str(alpha_char_as_slice(sep)) else {
                return FALSE;
            };
            return trie.branch_in_tail(s, &tail_str, data).into();
        }
        if *p == 0 {
            break;
        }
        p = p.offset(1);
    }

    // duplicated, overwrite val if flagged
    if !is_overwrite {
        return FALSE;
    }
    trie.tail.set_data(t, Some(data));
    trie.is_dirty.set(true);
    TRUE
}

#[deprecated(note = "Use trie.branch_in_branch()")]
fn trie_branch_in_branch(
    mut trie: NonNull<Trie>,
    sep_node: TrieIndex,
    suffix: *const TrieChar,
    data: TrieData,
) -> Bool {
    let trie = unsafe { trie.as_mut() };
    let suffix_slice = trie_char_as_slice(suffix);
    trie.branch_in_branch(sep_node, suffix_slice, data).into()
}

#[deprecated(note = "Use trie.branch_in_tail()")]
fn trie_branch_in_tail(
    mut trie: NonNull<Trie>,
    sep_node: TrieIndex,
    suffix: *const TrieChar,
    data: TrieData,
) -> Bool {
    let trie = unsafe { trie.as_mut() };
    let suffix_slice = trie_char_as_slice(suffix);
    trie.branch_in_tail(sep_node, suffix_slice, data).into()
}

#[no_mangle]
pub unsafe extern "C" fn trie_delete(mut trie: *mut Trie, mut key: *const AlphaChar) -> Bool {
    let mut trie = unsafe { &mut *trie };
    let mut s: TrieIndex = 0;
    let mut t: TrieIndex = 0;
    let mut suffix_idx: libc::c_short = 0;
    let mut p: *const AlphaChar = 0 as *const AlphaChar;
    s = da_get_root(&trie.da);
    p = key;
    while !(da_get_base(&trie.da, s) < 0 as libc::c_int) {
        let mut tc: TrieIndex = alpha_map_char_to_trie(&trie.alpha_map, *p);
        if TRIE_INDEX_MAX == tc {
            return FALSE as Bool;
        }
        if !da_walk(&trie.da, &mut s, tc as TrieChar) {
            return FALSE as Bool;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    t = -da_get_base(&trie.da, s);
    suffix_idx = 0 as libc::c_int as libc::c_short;
    loop {
        let mut tc_0: TrieIndex = alpha_map_char_to_trie(&trie.alpha_map, *p);
        if TRIE_INDEX_MAX == tc_0 {
            return FALSE as Bool;
        }
        if !tail_walk_char(&trie.tail, t, &mut suffix_idx, tc_0 as TrieChar) {
            return FALSE as Bool;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    tail_delete((&trie.tail).into(), t);
    da_set_base((&trie.da).into(), s, TRIE_INDEX_ERROR);
    da_prune((&trie.da).into(), s);
    trie.is_dirty.set(true);
    return TRUE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn trie_enumerate(
    mut trie: *mut Trie,
    mut enum_func: TrieEnumFunc,
    mut user_data: *mut libc::c_void,
) -> Bool {
    let mut root: *mut TrieState = 0 as *mut TrieState;
    let mut iter: *mut TrieIterator = 0 as *mut TrieIterator;
    let mut cont: Bool = TRUE as Bool;
    root = trie_root(trie);
    if root.is_null() {
        return FALSE as Bool;
    }
    iter = trie_iterator_new(root);
    if iter.is_null() {
        trie_state_free(root);
        return FALSE as Bool;
    } else {
        while cont.into() && trie_iterator_next(iter).into() {
            let mut key: *mut AlphaChar = trie_iterator_get_key(iter);
            let mut data: TrieData = trie_iterator_get_data(iter);
            cont = (Some(enum_func.expect("non-null function pointer")))
                .expect("non-null function pointer")(key, data, user_data);
            free(key as *mut libc::c_void);
        }
        trie_iterator_free(iter);
        trie_state_free(root);
        return cont;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trie_root(mut trie: *mut Trie) -> *mut TrieState {
    return trie_state_new(
        trie,
        da_get_root(&unsafe { &*trie }.da),
        0 as libc::c_int as libc::c_short,
        Into::<u32>::into(FALSE) as libc::c_short,
    );
}
unsafe extern "C" fn trie_state_new(
    mut trie: *mut Trie,
    mut index: TrieIndex,
    mut suffix_idx: libc::c_short,
    mut is_suffix: libc::c_short,
) -> *mut TrieState {
    let mut s: *mut TrieState = 0 as *mut TrieState;
    s = malloc(::core::mem::size_of::<TrieState>() as libc::c_ulong) as *mut TrieState;
    if s.is_null() {
        return NULL as *mut TrieState;
    }
    (*s).trie = trie;
    (*s).index = index;
    (*s).suffix_idx = suffix_idx;
    (*s).is_suffix = is_suffix;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_copy(mut dst: *mut TrieState, mut src: *const TrieState) {
    *dst = *src;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_clone(mut s: *const TrieState) -> *mut TrieState {
    return trie_state_new((*s).trie, (*s).index, (*s).suffix_idx, (*s).is_suffix);
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_free(mut s: *mut TrieState) {
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_rewind(mut s: *mut TrieState) {
    (*s).index = da_get_root(&(*(*s).trie).da);
    (*s).is_suffix = Into::<u32>::into(FALSE) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_walk(mut s: *mut TrieState, mut c: AlphaChar) -> Bool {
    let mut tc: TrieIndex = alpha_map_char_to_trie(&(*(*s).trie).alpha_map, c);
    if 0x7fffffff as libc::c_int == tc {
        return FALSE as Bool;
    }
    if (*s).is_suffix == 0 {
        let mut ret: Bool = DA_FALSE;
        ret = da_walk(&(*(*s).trie).da, &mut (*s).index, tc as TrieChar);
        if ret.into() && da_get_base(&(*(*s).trie).da, (*s).index) < 0 as libc::c_int {
            (*s).index = -da_get_base(&(*(*s).trie).da, (*s).index);
            (*s).suffix_idx = 0 as libc::c_int as libc::c_short;
            (*s).is_suffix = Into::<u32>::into(TRUE) as libc::c_short;
        }
        return ret;
    } else {
        return tail_walk_char(
            &(*(*s).trie).tail,
            (*s).index,
            &mut (*s).suffix_idx,
            tc as TrieChar,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_is_walkable(mut s: *const TrieState, mut c: AlphaChar) -> Bool {
    let mut tc: TrieIndex = alpha_map_char_to_trie(&(*(*s).trie).alpha_map, c);
    if 0x7fffffff as libc::c_int == tc {
        return FALSE as Bool;
    }
    if (*s).is_suffix == 0 {
        return (da_get_check(
            &(*(*s).trie).da,
            da_get_base(&(*(*s).trie).da, (*s).index) + tc as TrieChar as libc::c_int,
        ) == (*s).index)
            .into();
    } else {
        return (*(tail_get_suffix(&(*(*s).trie).tail, (*s).index)).offset((*s).suffix_idx as isize)
            as libc::c_int
            == tc as TrieChar as libc::c_int)
            .into();
    };
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_walkable_chars(
    mut s: *const TrieState,
    mut chars: *mut AlphaChar,
    mut chars_nelm: libc::c_int,
) -> libc::c_int {
    let mut syms_num: libc::c_int = 0 as libc::c_int;
    if (*s).is_suffix == 0 {
        let mut syms = da_output_symbols(&(*(*s).trie).da, (*s).index);
        let mut i: libc::c_int = 0;
        syms_num = syms.num() as libc::c_int;
        i = 0 as libc::c_int;
        while i < syms_num && i < chars_nelm {
            let mut tc: TrieChar = syms.get(i as usize).unwrap();
            *chars.offset(i as isize) = alpha_map_trie_to_char(&(*(*s).trie).alpha_map, tc);
            i += 1;
        }
    } else {
        let mut suffix: *const TrieChar = tail_get_suffix(&(*(*s).trie).tail, (*s).index);
        *chars.offset(0 as libc::c_int as isize) = alpha_map_trie_to_char(
            &(*(*s).trie).alpha_map,
            *suffix.offset((*s).suffix_idx as isize),
        );
        syms_num = 1 as libc::c_int;
    }
    return syms_num;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_is_single(mut s: *const TrieState) -> Bool {
    return (*s).is_suffix.into();
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_get_data(mut s: *const TrieState) -> TrieData {
    if s.is_null() {
        return TRIE_DATA_ERROR;
    }
    if (*s).is_suffix == 0 {
        let mut index: TrieIndex = (*s).index;
        if da_walk(&(*(*s).trie).da, &mut index, TRIE_CHAR_TERM as TrieChar).into() {
            if da_get_base(&(*(*s).trie).da, index) < 0 as libc::c_int {
                index = -da_get_base(&(*(*s).trie).da, index);
                return tail_get_data(&(*(*s).trie).tail, index);
            }
        }
    } else if *(tail_get_suffix(&(*(*s).trie).tail, (*s).index)).offset((*s).suffix_idx as isize)
        as libc::c_int
        == '\0' as i32
    {
        return tail_get_data(&(*(*s).trie).tail, (*s).index);
    }
    return TRIE_DATA_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_new(mut s: *mut TrieState) -> *mut TrieIterator {
    let mut iter: *mut TrieIterator = 0 as *mut TrieIterator;
    iter = malloc(::core::mem::size_of::<TrieIterator>() as libc::c_ulong) as *mut TrieIterator;
    if iter.is_null() {
        return NULL as *mut TrieIterator;
    }
    (*iter).root = s;
    (*iter).state = NULL as *mut TrieState;
    (*iter).key = NULL as *mut TrieString;
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_free(mut iter: *mut TrieIterator) {
    if !((*iter).state).is_null() {
        trie_state_free((*iter).state);
    }
    if !((*iter).key).is_null() {
        trie_string_free((*iter).key);
    }
    free(iter as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_next(mut iter: *mut TrieIterator) -> Bool {
    let mut s: *mut TrieState = (*iter).state;
    let mut sep: TrieIndex = 0;
    if s.is_null() {
        (*iter).state = trie_state_clone((*iter).root);
        s = (*iter).state;
        if (*s).is_suffix != 0 {
            return TRUE as Bool;
        }
        (*iter).key = trie_string_new(20 as libc::c_int);
        sep = da_first_separate(
            NonNull::new_unchecked(&mut (*(*s).trie).da),
            (*s).index,
            NonNull::new_unchecked((*iter).key),
        );
        if TRIE_INDEX_ERROR == sep {
            return FALSE as Bool;
        }
        (*s).index = sep;
        return TRUE as Bool;
    }
    if (*s).is_suffix != 0 {
        return FALSE as Bool;
    }
    sep = da_next_separate(
        NonNull::new_unchecked(&mut (*(*s).trie).da),
        (*(*iter).root).index,
        (*s).index,
        NonNull::new_unchecked((*iter).key),
    );
    if TRIE_INDEX_ERROR == sep {
        return FALSE as Bool;
    }
    (*s).index = sep;
    return TRUE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_get_key(mut iter: *const TrieIterator) -> *mut AlphaChar {
    let mut s: *const TrieState = 0 as *const TrieState;
    let mut tail_str: *const TrieChar = 0 as *const TrieChar;
    let mut alpha_key: *mut AlphaChar = 0 as *mut AlphaChar;
    let mut alpha_p: *mut AlphaChar = 0 as *mut AlphaChar;
    s = (*iter).state;
    if s.is_null() {
        return NULL as *mut AlphaChar;
    }
    if (*s).is_suffix != 0 {
        tail_str = tail_get_suffix(&(*(*s).trie).tail, (*s).index);
        if tail_str.is_null() {
            return NULL as *mut AlphaChar;
        }
        tail_str = tail_str.offset((*s).suffix_idx as libc::c_int as isize);
        alpha_key = malloc(
            (::core::mem::size_of::<AlphaChar>() as libc::c_ulong).wrapping_mul(
                (trie_char_strlen(tail_str) as size_t).wrapping_add(1 as libc::c_int as size_t),
            ),
        ) as *mut AlphaChar;
        alpha_p = alpha_key;
    } else {
        let mut tail_idx: TrieIndex = 0;
        let mut i: libc::c_int = 0;
        let mut key_len: libc::c_int = 0;
        let mut key_p: *const TrieChar = 0 as *const TrieChar;
        tail_idx = -da_get_base(&(*(*s).trie).da, (*s).index);
        tail_str = tail_get_suffix(&(*(*s).trie).tail, tail_idx);
        if tail_str.is_null() {
            return NULL as *mut AlphaChar;
        }
        key_len = trie_string_length((*iter).key);
        key_p = trie_string_get_val((*iter).key) as *const TrieChar;
        alpha_key = malloc(
            (::core::mem::size_of::<AlphaChar>() as libc::c_ulong).wrapping_mul(
                (key_len as size_t)
                    .wrapping_add(trie_char_strlen(tail_str) as size_t)
                    .wrapping_add(1 as libc::c_int as size_t),
            ),
        ) as *mut AlphaChar;
        alpha_p = alpha_key;
        i = key_len;
        while i > 0 as libc::c_int {
            let fresh0 = key_p;
            key_p = key_p.offset(1);
            let fresh1 = alpha_p;
            alpha_p = alpha_p.offset(1);
            *fresh1 = alpha_map_trie_to_char(&(*(*s).trie).alpha_map, *fresh0);
            i -= 1;
            i;
        }
    }
    while TRIE_CHAR_TERM != *tail_str {
        let fresh2 = tail_str;
        tail_str = tail_str.offset(1);
        let fresh3 = alpha_p;
        alpha_p = alpha_p.offset(1);
        *fresh3 = alpha_map_trie_to_char(&(*(*s).trie).alpha_map, *fresh2);
    }
    *alpha_p = 0 as libc::c_int as AlphaChar;
    return alpha_key;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_get_data(mut iter: *const TrieIterator) -> TrieData {
    let mut s: *const TrieState = (*iter).state;
    let mut tail_index: TrieIndex = 0;
    if s.is_null() {
        return TRIE_DATA_ERROR;
    }
    if (*s).is_suffix == 0 {
        if !(da_get_base(&(*(*s).trie).da, (*s).index) < 0 as libc::c_int) {
            return TRIE_DATA_ERROR;
        }
        tail_index = -da_get_base(&(*(*s).trie).da, (*s).index);
    } else {
        tail_index = (*s).index;
    }
    return tail_get_data(&(*(*s).trie).tail, tail_index);
}

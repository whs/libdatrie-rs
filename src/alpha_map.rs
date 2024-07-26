use ::libc;
use byteorder::{BigEndian, ReadBytesExt};
use null_terminated::Nul;
use std::cmp::Ordering;
use std::io::{Read, Seek, SeekFrom};
use std::{io, iter, ptr, slice};

use crate::alpha_range::{AlphaRange, AlphaRangeIter, AlphaRangeIterMut};
use crate::fileutils::*;
use crate::trie_string::{trie_char_strlen, TrieChar, TRIE_CHAR_TERM};

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub type FILE = libc::FILE;
pub type uint8 = u8;
pub type uint32 = u32;
pub type int32 = i32;
pub type size_t = usize;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;

pub type TrieIndex = i32;
pub const TRIE_INDEX_MAX: TrieIndex = 0x7fffffff;

pub type AlphaChar = u32;
pub const ALPHA_CHAR_ERROR: AlphaChar = AlphaChar::MAX;

// TODO: Check whether the input is required or not, and change type accordingly

#[no_mangle]
pub extern "C" fn alpha_char_strlen(str: *const AlphaChar) -> i32 {
    // TODO: Use memchr
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

#[repr(C)]
pub struct AlphaMap {
    pub alpha_begin: AlphaChar,
    pub first_range: *mut AlphaRange,
    pub alpha_end: AlphaChar,
    pub alpha_map_sz: i32,
    pub alpha_to_trie_map: *mut TrieIndex,
    pub trie_map_sz: i32,
    pub trie_to_alpha_map: *mut AlphaChar,
}

pub const ALPHAMAP_SIGNATURE: u32 = 0xd9fcd9fc;

impl AlphaMap {
    fn range_iter(&self) -> Option<AlphaRangeIter> {
        unsafe { self.first_range.as_ref().map(|v| v.iter()) }
    }

    fn range_iter_mut(&self) -> Option<AlphaRangeIterMut> {
        unsafe { self.first_range.as_mut().map(|v| v.iter_mut()) }
    }

    fn alpha_to_trie_map_slice(&self) -> Option<&[TrieIndex]> {
        unsafe {
            self.alpha_to_trie_map
                .as_ref()
                .map(|v| slice::from_raw_parts(v, self.alpha_map_sz as usize))
        }
    }

    fn alpha_to_trie_map_slice_mut(&self) -> Option<&mut [TrieIndex]> {
        unsafe {
            self.alpha_to_trie_map
                .as_mut()
                .map(|v| slice::from_raw_parts_mut(v, self.alpha_map_sz as usize))
        }
    }

    fn trie_to_alpha_map_slice(&self) -> Option<&[AlphaChar]> {
        unsafe {
            self.trie_to_alpha_map
                .as_ref()
                .map(|v| slice::from_raw_parts(v, self.trie_map_sz as usize))
        }
    }

    fn trie_to_alpha_map_slice_mut(&self) -> Option<&mut [AlphaChar]> {
        unsafe {
            self.trie_to_alpha_map
                .as_mut()
                .map(|v| slice::from_raw_parts_mut(v, self.trie_map_sz as usize))
        }
    }
}

impl Default for AlphaMap {
    fn default() -> Self {
        AlphaMap {
            first_range: ptr::null_mut(),
            alpha_begin: 0,
            alpha_end: 0,
            alpha_map_sz: 0,
            alpha_to_trie_map: ptr::null_mut(),
            trie_map_sz: 0,
            trie_to_alpha_map: ptr::null_mut(),
        }
    }
}

impl Clone for AlphaMap {
    fn clone(&self) -> Self {
        let mut am = Self::default();

        if let Some(iter) = self.range_iter() {
            for range in iter {
                unsafe {
                    if alpha_map_add_range_only(&mut am, range.begin, range.end) != 0 {
                        panic!("clone fail")
                    }
                }
            }
        }

        unsafe {
            if alpha_map_recalc_work_area(&mut am) != 0 {
                panic!("clone fail")
            }
        }

        am
    }
}

impl Drop for AlphaMap {
    fn drop(&mut self) {
        unsafe {
            let mut p = self.first_range;
            while !p.is_null() {
                let q = (*p).next;
                free(p as *mut libc::c_void);
                p = q;
            }
            if !self.alpha_to_trie_map.is_null() {
                free(self.alpha_to_trie_map as *mut libc::c_void);
            }
            if !self.trie_to_alpha_map.is_null() {
                free(self.trie_to_alpha_map as *mut libc::c_void);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn alpha_map_new() -> *mut AlphaMap {
    Box::into_raw(Box::new(AlphaMap::default()))
}

#[no_mangle]
pub extern "C" fn alpha_map_clone(mut a_map: *const AlphaMap) -> *mut AlphaMap {
    let Some(am) = (unsafe { a_map.as_ref() }) else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(am.clone()))
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_free(alpha_map: *mut AlphaMap) {
    let am = Box::from_raw(alpha_map);
    drop(am) // This is not strictly needed, but it helps in clarity
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_fread_bin(file: *mut libc::FILE) -> *mut AlphaMap {
    let Some(mut file) = wrap_cfile(file) else {
        return ptr::null_mut();
    };

    let save_pos = file.seek(SeekFrom::Current(0)).unwrap();

    match _read(&mut file) {
        Ok(am) => Box::into_raw(Box::new(am.clone())),
        Err(_) => {
            // Return to save_pos if read fail
            let _ = file.seek(SeekFrom::Start(save_pos));
            return ptr::null_mut();
        }
    }
}

fn _read<T: Read>(stream: &mut T) -> io::Result<AlphaMap> {
    // check signature
    match stream.read_u32::<BigEndian>() {
        Ok(ALPHAMAP_SIGNATURE) => {}
        Ok(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid signature",
            ))
        }
        Err(v) => return Err(v),
    }

    let mut alphamap = AlphaMap::default();

    // Read number of ranges
    let total = stream.read_i32::<BigEndian>()?;

    // Read character ranges
    for _ in 0..total {
        let b = stream.read_i32::<BigEndian>()?;
        let e = stream.read_i32::<BigEndian>()?;
        unsafe {
            alpha_map_add_range_only(&mut alphamap, b as AlphaChar, e as AlphaChar);
        }
    }

    // work area
    unsafe {
        if alpha_map_recalc_work_area(&mut alphamap) != 0 {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "alpha_map_recalc_work_area fail",
            ));
        }
    }

    Ok(alphamap)
}

// TODO: Change to usize
fn alpha_map_get_total_ranges(alpha_map: *const AlphaMap) -> i32 {
    let am = unsafe { &*alpha_map };

    am.range_iter().map(|iter| iter.count()).unwrap_or(0) as i32
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_fwrite_bin(
    mut alpha_map: *const AlphaMap,
    mut file: *mut FILE,
) -> libc::c_int {
    let mut range: *mut AlphaRange = 0 as *mut AlphaRange;
    if file_write_int32(file, ALPHAMAP_SIGNATURE as int32) as u64 == 0
        || file_write_int32(file, alpha_map_get_total_ranges(alpha_map)) as u64 == 0
    {
        return -(1 as libc::c_int);
    }
    range = (*alpha_map).first_range;
    while !range.is_null() {
        if file_write_int32(file, (*range).begin as int32) as u64 == 0
            || file_write_int32(file, (*range).end as int32) as u64 == 0
        {
            return -(1 as libc::c_int);
        }
        range = (*range).next;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_get_serialized_size(alpha_map: *const AlphaMap) -> usize {
    let ranges_count = alpha_map_get_total_ranges(alpha_map);

    return 4 // ALPHAMAP_SIGNATURE
    + size_of::<i32>() // ranges_count
    + (size_of::<AlphaChar>() * 2 * ranges_count as usize);
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_serialize_bin(
    mut alpha_map: *const AlphaMap,
    mut ptr: *mut *mut uint8,
) {
    let mut range: *mut AlphaRange = 0 as *mut AlphaRange;
    serialize_int32_be_incr(ptr, ALPHAMAP_SIGNATURE as int32);
    serialize_int32_be_incr(ptr, alpha_map_get_total_ranges(alpha_map));
    range = (*alpha_map).first_range;
    while !range.is_null() {
        serialize_int32_be_incr(ptr, (*range).begin as int32);
        serialize_int32_be_incr(ptr, (*range).end as int32);
        range = (*range).next;
    }
}
unsafe extern "C" fn alpha_map_add_range_only(
    alpha_map: *mut AlphaMap,
    begin: AlphaChar,
    end: AlphaChar,
) -> libc::c_int {
    if begin > end {
        return -1;
    }
    let mut begin_node = 0 as *mut AlphaRange;
    let mut end_node = 0 as *mut AlphaRange;
    let mut q = 0 as *mut AlphaRange;
    let mut r = (*alpha_map).first_range;
    while !r.is_null() && (*r).begin <= begin {
        if begin <= (*r).end {
            begin_node = r;
            break;
        } else if (*r).end + 1 == begin {
            (*r).end = begin;
            begin_node = r;
            break;
        } else {
            q = r;
            r = (*r).next;
        }
    }
    if begin_node.is_null() && !r.is_null() && (*r).begin <= end + 1 {
        (*r).begin = begin;
        begin_node = r;
    }
    while !r.is_null() && (*r).begin <= end + 1 {
        if end <= (*r).end {
            end_node = r;
        } else if r != begin_node {
            if !q.is_null() {
                (*q).next = (*r).next;
                free(r as *mut libc::c_void);
                r = (*q).next;
            } else {
                (*alpha_map).first_range = (*r).next;
                free(r as *mut libc::c_void);
                r = (*alpha_map).first_range;
            }
            continue;
        }
        q = r;
        r = (*r).next;
    }
    if end_node.is_null() && !q.is_null() && begin <= (*q).end {
        (*q).end = end;
        end_node = q;
    }
    if !begin_node.is_null() && !end_node.is_null() {
        if begin_node != end_node {
            if (*begin_node).next == end_node {
            } else {
                // __assert_fail(
                //     b"begin_node->next == end_node\0" as *const u8 as *const libc::c_char,
                //     b"../datrie/alpha-map.c\0" as *const u8 as *const libc::c_char,
                //     396 as libc::c_int as libc::c_uint,
                //     __ASSERT_FUNCTION.as_ptr(),
                // );
                panic!("Assert_fail")
            }
            'c_2743: {
                if (*begin_node).next == end_node {
                } else {
                    // __assert_fail(
                    //     b"begin_node->next == end_node\0" as *const u8 as *const libc::c_char,
                    //     b"../datrie/alpha-map.c\0" as *const u8 as *const libc::c_char,
                    //     396 as libc::c_int as libc::c_uint,
                    //     __ASSERT_FUNCTION.as_ptr(),
                    // );
                    panic!("Assert_fail")
                }
            };
            (*begin_node).end = (*end_node).end;
            (*begin_node).next = (*end_node).next;
            free(end_node as *mut libc::c_void);
        }
    } else if begin_node.is_null() && end_node.is_null() {
        let mut range: *mut AlphaRange =
            malloc(size_of::<AlphaRange>() as libc::c_ulong) as *mut AlphaRange;
        if range.is_null() {
            return -1;
        }
        (*range).begin = begin;
        (*range).end = end;
        if !q.is_null() {
            (*q).next = range;
        } else {
            (*alpha_map).first_range = range;
        }
        (*range).next = r;
    }
    0
}

unsafe extern "C" fn alpha_map_recalc_work_area(mut alpha_map: *mut AlphaMap) -> libc::c_int {
    let mut current_block: u64;
    let mut range: *mut AlphaRange = 0 as *mut AlphaRange;
    if !((*alpha_map).alpha_to_trie_map).is_null() {
        free((*alpha_map).alpha_to_trie_map as *mut libc::c_void);
        (*alpha_map).alpha_to_trie_map = NULL as *mut TrieIndex;
    }
    if !((*alpha_map).trie_to_alpha_map).is_null() {
        free((*alpha_map).trie_to_alpha_map as *mut libc::c_void);
        (*alpha_map).trie_to_alpha_map = NULL as *mut AlphaChar;
    }
    range = (*alpha_map).first_range;
    if !range.is_null() {
        let alpha_begin: AlphaChar = (*range).begin;
        let mut n_alpha: libc::c_int = 0;
        let mut n_trie: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut a: AlphaChar = 0;
        let mut trie_char: TrieIndex = 0;
        (*alpha_map).alpha_begin = alpha_begin;
        n_trie = 0 as libc::c_int;
        loop {
            n_trie = (n_trie as AlphaChar).wrapping_add(
                ((*range).end)
                    .wrapping_sub((*range).begin)
                    .wrapping_add(1 as libc::c_int as AlphaChar),
            ) as libc::c_int as libc::c_int;
            if ((*range).next).is_null() {
                break;
            }
            range = (*range).next;
        }
        if n_trie < TRIE_CHAR_TERM as i32 {
            n_trie = (TRIE_CHAR_TERM + 1) as libc::c_int;
        } else {
            n_trie += 1;
            n_trie;
        }
        (*alpha_map).alpha_end = (*range).end;
        n_alpha = ((*range).end)
            .wrapping_sub(alpha_begin)
            .wrapping_add(1 as libc::c_int as AlphaChar) as libc::c_int;
        (*alpha_map).alpha_map_sz = n_alpha;
        (*alpha_map).alpha_to_trie_map = malloc(
            (n_alpha as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TrieIndex>() as libc::c_ulong),
        ) as *mut TrieIndex;
        if ((*alpha_map).alpha_to_trie_map).is_null() {
            current_block = 1868236917207382637;
        } else {
            i = 0 as libc::c_int;
            while i < n_alpha {
                *((*alpha_map).alpha_to_trie_map).offset(i as isize) = TRIE_INDEX_MAX;
                i += 1;
                i;
            }
            (*alpha_map).trie_map_sz = n_trie;
            (*alpha_map).trie_to_alpha_map = malloc(
                (n_trie as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<AlphaChar>() as libc::c_ulong),
            ) as *mut AlphaChar;
            if ((*alpha_map).trie_to_alpha_map).is_null() {
                free((*alpha_map).alpha_to_trie_map as *mut libc::c_void);
                (*alpha_map).alpha_to_trie_map = NULL as *mut TrieIndex;
                current_block = 1868236917207382637;
            } else {
                trie_char = 0 as libc::c_int;
                range = (*alpha_map).first_range;
                while !range.is_null() {
                    a = (*range).begin;
                    while a <= (*range).end {
                        if TRIE_CHAR_TERM as TrieIndex == trie_char {
                            trie_char += 1;
                            trie_char;
                        }
                        *((*alpha_map).alpha_to_trie_map)
                            .offset(a.wrapping_sub(alpha_begin) as isize) = trie_char;
                        *((*alpha_map).trie_to_alpha_map).offset(trie_char as isize) = a;
                        trie_char += 1;
                        trie_char;
                        a = a.wrapping_add(1);
                        a;
                    }
                    range = (*range).next;
                }
                while trie_char < n_trie {
                    let fresh0 = trie_char;
                    trie_char = trie_char + 1;
                    *((*alpha_map).trie_to_alpha_map).offset(fresh0 as isize) = ALPHA_CHAR_ERROR;
                }
                *((*alpha_map).trie_to_alpha_map).offset(TRIE_CHAR_TERM as isize) =
                    0 as libc::c_int as AlphaChar;
                current_block = 572715077006366937;
            }
        }
        match current_block {
            572715077006366937 => {}
            _ => return -(1 as libc::c_int),
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_add_range(
    mut alpha_map: *mut AlphaMap,
    mut begin: AlphaChar,
    mut end: AlphaChar,
) -> libc::c_int {
    let mut res: libc::c_int = alpha_map_add_range_only(alpha_map, begin, end);
    if res != 0 as libc::c_int {
        return res;
    }
    return alpha_map_recalc_work_area(alpha_map);
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_char_to_trie(
    alpha_map: *const AlphaMap,
    ac: AlphaChar,
) -> TrieIndex {
    if ac == 0 {
        return TRIE_CHAR_TERM as TrieIndex;
    }

    let am = unsafe { &*alpha_map };
    let Some(alpha_to_trie) = am.alpha_to_trie_map_slice() else {
        return TRIE_INDEX_MAX;
    };

    if (am.alpha_begin..=am.alpha_end).contains(&ac) {
        // TODO: We probably can write better mapping
        return alpha_to_trie[(ac - am.alpha_begin) as usize];
    }

    TRIE_INDEX_MAX
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_trie_to_char(
    alpha_map: *const AlphaMap,
    tc: TrieChar,
) -> AlphaChar {
    let am = unsafe { &(*alpha_map) };
    am.trie_to_alpha_map_slice()
        .map(|v| v.get(tc as usize))
        .flatten()
        .copied()
        .unwrap_or(ALPHA_CHAR_ERROR)
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_char_to_trie_str(
    alpha_map: *const AlphaMap,
    mut str: *const AlphaChar,
) -> *mut TrieChar {
    let str = unsafe { slice::from_raw_parts(str, alpha_char_strlen(str) as usize) };

    let out_vec: Option<Vec<TrieChar>> = str
        .iter()
        .map(|v| {
            let tc = alpha_map_char_to_trie(alpha_map, *v);
            if tc == TRIE_INDEX_MAX {
                return None;
            }
            Some(tc as TrieChar)
        })
        .chain(iter::once(Some(TRIE_CHAR_TERM)))
        .collect();

    match out_vec {
        Some(v) => Box::into_raw(v.into_boxed_slice()).cast(),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_trie_to_char_str(
    alpha_map: *const AlphaMap,
    str: *const TrieChar,
) -> *mut AlphaChar {
    let str = unsafe { slice::from_raw_parts(str, trie_char_strlen(str)) };

    let out: Vec<AlphaChar> = str
        .iter()
        .map(|chr| alpha_map_trie_to_char(alpha_map, *chr))
        .chain(iter::once(0))
        .collect();

    Box::into_raw(out.into_boxed_slice()).cast()
}

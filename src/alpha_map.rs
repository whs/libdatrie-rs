use crate::fileutils::*;
use crate::trie_string::{trie_char_strlen, TrieChar, TRIE_CHAR_TERM};
use ::libc;
use null_terminated::Nul;
use std::cmp::Ordering;
use std::{iter, ptr, slice};
use std::marker::PhantomData;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlphaRange {
    pub next: *mut AlphaRange,
    pub begin: AlphaChar,
    pub end: AlphaChar,
}

impl AlphaRange {
    pub fn iter(&self) -> impl Iterator<Item=&AlphaRange> {
        AlphaRangeIter{
            range: self,
            phantom: PhantomData,
        }
    }
}

struct AlphaRangeIter<'a> {
    range: *const AlphaRange,
    phantom: PhantomData<&'a AlphaRange>,
}

impl<'a> Iterator for AlphaRangeIter<'a> {
    type Item = &'a AlphaRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.is_null() {
            return None;
        }
        let out = unsafe { &*self.range };
        self.range = out.next;
        Some(out)
    }
}

pub type AlphaChar = u32;
pub const ALPHA_CHAR_ERROR: AlphaChar = AlphaChar::MAX;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlphaMap {
    pub first_range: *mut AlphaRange,
    pub alpha_begin: AlphaChar,
    pub alpha_end: AlphaChar,
    pub alpha_map_sz: i32,
    pub alpha_to_trie_map: *mut TrieIndex,
    pub trie_map_sz: i32,
    pub trie_to_alpha_map: *mut AlphaChar,
}
pub const ALPHAMAP_SIGNATURE: u32 = 0xd9fcd9fc;

impl AlphaMap {
    fn range_iter(&self) -> Option<impl Iterator<Item = &AlphaRange>> {
        if self.first_range.is_null() {
            return None;
        }

        unsafe { return Some((&*self.first_range).iter()) }
    }

    fn alpha_to_trie_map_slice(&self) -> Option<&[TrieIndex]> {
        if self.alpha_to_trie_map.is_null() {
            return None;
        }

        unsafe {
            Some(slice::from_raw_parts(
                self.alpha_to_trie_map,
                self.alpha_map_sz as usize,
            ))
        }
    }

    fn alpha_to_trie_map_slice_mut(&self) -> Option<&mut [TrieIndex]> {
        if self.alpha_to_trie_map.is_null() {
            return None;
        }

        unsafe {
            Some(slice::from_raw_parts_mut(
                self.alpha_to_trie_map,
                self.alpha_map_sz as usize,
            ))
        }
    }

    fn trie_to_alpha_map_slice(&self) -> Option<&[AlphaChar]> {
        if self.trie_to_alpha_map.is_null() {
            return None;
        }

        unsafe {
            Some(slice::from_raw_parts(
                self.trie_to_alpha_map,
                self.trie_map_sz as usize,
            ))
        }
    }

    fn trie_to_alpha_map_slice_mut(&self) -> Option<&mut [AlphaChar]> {
        if self.trie_to_alpha_map.is_null() {
            return None;
        }

        unsafe {
            Some(slice::from_raw_parts_mut(
                self.trie_to_alpha_map,
                self.trie_map_sz as usize,
            ))
        }
    }
}

#[no_mangle]
pub extern "C" fn alpha_map_new() -> *mut AlphaMap {
    let am = AlphaMap {
        first_range: ptr::null_mut(),
        alpha_begin: 0,
        alpha_end: 0,
        alpha_map_sz: 0,
        alpha_to_trie_map: ptr::null_mut(),
        trie_map_sz: 0,
        trie_to_alpha_map: ptr::null_mut(),
    };

    Box::into_raw(Box::new(am))
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_clone(mut a_map: *const AlphaMap) -> *mut AlphaMap {
    let mut current_block: u64;
    let mut alpha_map: *mut AlphaMap = 0 as *mut AlphaMap;
    let mut range: *mut AlphaRange = 0 as *mut AlphaRange;
    alpha_map = alpha_map_new();
    if alpha_map.is_null() {
        return NULL as *mut AlphaMap;
    }
    range = (*a_map).first_range;
    loop {
        if range.is_null() {
            current_block = 15619007995458559411;
            break;
        }
        if alpha_map_add_range_only(alpha_map, (*range).begin, (*range).end) != 0 as libc::c_int {
            current_block = 14813426389682942902;
            break;
        }
        range = (*range).next;
    }
    match current_block {
        15619007995458559411 => {
            if !(alpha_map_recalc_work_area(alpha_map) != 0 as libc::c_int) {
                return alpha_map;
            }
        }
        _ => {}
    }
    alpha_map_free(alpha_map);
    return NULL as *mut AlphaMap;
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_free(alpha_map: *mut AlphaMap) {
    let am = Box::from_raw(alpha_map);

    let mut p = am.first_range;
    while !p.is_null() {
        let q = (*p).next;
        free(p as *mut libc::c_void);
        p = q;
    }
    if !am.alpha_to_trie_map.is_null() {
        free(am.alpha_to_trie_map as *mut libc::c_void);
    }
    if !am.trie_to_alpha_map.is_null() {
        free(am.trie_to_alpha_map as *mut libc::c_void);
    }

    drop(am) // This is not strictly needed, but it help in clarity
}

#[no_mangle]
pub unsafe extern "C" fn alpha_map_fread_bin(mut file: *mut FILE) -> *mut AlphaMap {
    let mut current_block: u64;
    let mut save_pos: libc::c_long = 0;
    let mut sig: uint32 = 0;
    let mut total: int32 = 0;
    let mut i: int32 = 0;
    let mut alpha_map: *mut AlphaMap = 0 as *mut AlphaMap;
    save_pos = ftell(file);
    if !(file_read_int32(file, &mut sig as *mut uint32 as *mut int32) as u64 == 0
        || ALPHAMAP_SIGNATURE != sig)
    {
        alpha_map = alpha_map_new();
        if !alpha_map.is_null() {
            if !(file_read_int32(file, &mut total) as u64 == 0) {
                i = 0 as libc::c_int;
                loop {
                    if !(i < total) {
                        current_block = 1917311967535052937;
                        break;
                    }
                    let mut b: int32 = 0;
                    let mut e: int32 = 0;
                    if file_read_int32(file, &mut b) as u64 == 0
                        || file_read_int32(file, &mut e) as u64 == 0
                    {
                        current_block = 7291624750341993849;
                        break;
                    }
                    alpha_map_add_range_only(alpha_map, b as AlphaChar, e as AlphaChar);
                    i += 1;
                    i;
                }
                match current_block {
                    7291624750341993849 => {}
                    _ => {
                        if !(alpha_map_recalc_work_area(alpha_map) != 0 as libc::c_int) {
                            return alpha_map;
                        }
                    }
                }
            }
            alpha_map_free(alpha_map);
        }
    }
    fseek(file, save_pos, SEEK_SET);
    return NULL as *mut AlphaMap;
}

fn alpha_map_get_total_ranges(alpha_map: *const AlphaMap) -> i32 {
    let am = unsafe { &*alpha_map };

    match am.range_iter() {
        Some(iter) => iter.count() as i32,
        None => 0,
    }
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
pub extern "C" fn alpha_map_get_serialized_size(alpha_map: *const AlphaMap) -> usize {
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
    mut alpha_map: *mut AlphaMap,
    mut begin: AlphaChar,
    mut end: AlphaChar,
) -> libc::c_int {
    let mut q: *mut AlphaRange = 0 as *mut AlphaRange;
    let mut r: *mut AlphaRange = 0 as *mut AlphaRange;
    let mut begin_node: *mut AlphaRange = 0 as *mut AlphaRange;
    let mut end_node: *mut AlphaRange = 0 as *mut AlphaRange;
    if begin > end {
        return -(1 as libc::c_int);
    }
    end_node = 0 as *mut AlphaRange;
    begin_node = end_node;
    q = 0 as *mut AlphaRange;
    r = (*alpha_map).first_range;
    while !r.is_null() && (*r).begin <= begin {
        if begin <= (*r).end {
            begin_node = r;
            break;
        } else if ((*r).end).wrapping_add(1 as libc::c_int as AlphaChar) == begin {
            (*r).end = begin;
            begin_node = r;
            break;
        } else {
            q = r;
            r = (*r).next;
        }
    }
    if begin_node.is_null()
        && !r.is_null()
        && (*r).begin <= end.wrapping_add(1 as libc::c_int as AlphaChar)
    {
        (*r).begin = begin;
        begin_node = r;
    }
    while !r.is_null() && (*r).begin <= end.wrapping_add(1 as libc::c_int as AlphaChar) {
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
            malloc(::core::mem::size_of::<AlphaRange>() as libc::c_ulong) as *mut AlphaRange;
        if range.is_null() {
            return -(1 as libc::c_int);
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
    return 0 as libc::c_int;
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
pub extern "C" fn alpha_map_char_to_trie(alpha_map: *const AlphaMap, ac: AlphaChar) -> TrieIndex {
    if ac == 0 {
        return TRIE_CHAR_TERM as TrieIndex;
    }

    let am = unsafe { &*alpha_map };
    let alpha_to_trie = match am.alpha_to_trie_map_slice() {
        Some(v) => v,
        None => return TRIE_INDEX_MAX,
    };

    if (am.alpha_begin..=am.alpha_end).contains(&ac) {
        // TODO: We probably can write better mapping
        return alpha_to_trie[(ac - am.alpha_begin) as usize];
    }

    TRIE_INDEX_MAX
}

#[no_mangle]
pub extern "C" fn alpha_map_trie_to_char(alpha_map: *const AlphaMap, tc: TrieChar) -> AlphaChar {
    let am = unsafe { &(*alpha_map) };
    am.trie_to_alpha_map_slice()
        .map(|v| v.get(tc as usize))
        .flatten()
        .copied()
        .unwrap_or(ALPHA_CHAR_ERROR)
}

#[no_mangle]
pub extern "C" fn alpha_map_char_to_trie_str(
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
pub extern "C" fn alpha_map_trie_to_char_str(
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

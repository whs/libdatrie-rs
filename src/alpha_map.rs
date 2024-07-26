use std::cmp::Ordering;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ptr::NonNull;
use std::{io, iter, ptr, slice};

use ::libc;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use null_terminated::Nul;

use crate::alpha_range::{AlphaRange, AlphaRangeIter, AlphaRangeIterMut};
use crate::fileutils::wrap_cfile_nonnull;
use crate::trie_string::{trie_char_strlen, TrieChar, TRIE_CHAR_TERM};

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}

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

#[repr(C)]
pub struct AlphaMap {
    alpha_begin: AlphaChar,
    first_range: *mut AlphaRange,
    alpha_end: AlphaChar,
    alpha_to_trie_map: Box<[TrieIndex]>,
    trie_to_alpha_map: Box<[AlphaChar]>,
}

pub const ALPHAMAP_SIGNATURE: u32 = 0xd9fcd9fc;

impl AlphaMap {
    pub(crate) fn read<T: Read>(stream: &mut T) -> io::Result<AlphaMap> {
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
                alpha_map_add_range_only((&mut alphamap).into(), b as AlphaChar, e as AlphaChar);
            }
        }

        // work area
        alphamap.recalc_work_area();

        Ok(alphamap)
    }

    fn recalc_work_area(&mut self) {
        // free old existing map
        self.alpha_to_trie_map = Box::new([]);
        self.trie_to_alpha_map = Box::new([]);

        let mut range = self.first_range;
        if range.is_null() {
            return;
        }

        // This is basically am.first_range[0].begin
        let alpha_begin = unsafe { (*range).begin };

        self.alpha_begin = alpha_begin;
        let mut n_trie: usize = self
            .range_iter()
            .unwrap()
            .map(|range| range.end as usize - range.begin as usize + 1)
            .sum();
        if n_trie < TRIE_CHAR_TERM as usize {
            // does this even hit? overflow handling?
            n_trie = TRIE_CHAR_TERM as usize + 1;
        } else {
            n_trie += 1;
        }
        self.alpha_end = self.range_iter().unwrap().last().unwrap().end;

        let n_alpha = self.alpha_end as usize - alpha_begin as usize + 1;

        let mut alpha_to_trie_map = vec![TRIE_INDEX_MAX; n_alpha].into_boxed_slice();
        let mut trie_to_alpha_map = vec![ALPHA_CHAR_ERROR; n_trie].into_boxed_slice();

        let mut trie_char: TrieIndex = 0;
        for range in unsafe { *self.first_range }.iter() {
            for a in range.begin..=range.end {
                if trie_char == TRIE_CHAR_TERM as TrieIndex {
                    trie_char += 1;
                }
                // This does bond check every loop iteration...
                alpha_to_trie_map[(a - alpha_begin) as usize] = trie_char as TrieIndex;
                trie_to_alpha_map[trie_char as usize] = a;
                trie_char += 1;
            }
        }
        trie_to_alpha_map[TRIE_CHAR_TERM as usize] = 0;

        self.alpha_to_trie_map = alpha_to_trie_map;
        self.trie_to_alpha_map = trie_to_alpha_map;
    }

    fn range_iter(&self) -> Option<AlphaRangeIter> {
        unsafe { self.first_range.as_ref().map(|v| v.iter()) }
    }

    fn range_iter_mut(&self) -> Option<AlphaRangeIterMut> {
        unsafe { self.first_range.as_mut().map(|v| v.iter_mut()) }
    }

    fn total_range(&self) -> usize {
        self.range_iter().map(|iter| iter.count()).unwrap_or(0)
    }

    fn serialize<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_u32::<BigEndian>(ALPHAMAP_SIGNATURE)?;
        buf.write_i32::<BigEndian>(self.total_range() as i32)?;

        if let Some(iter) = self.range_iter() {
            for range in iter {
                buf.write_i32::<BigEndian>(range.begin as i32)?;
                buf.write_i32::<BigEndian>(range.end as i32)?;
            }
        }

        Ok(())
    }

    pub(crate) fn serialized_size(&self) -> usize {
        return 4 // ALPHAMAP_SIGNATURE
            + size_of::<i32>() // ranges_count
            + (size_of::<AlphaChar>() * 2 * self.total_range());
    }

    pub(crate) fn char_to_trie(&self, ac: AlphaChar) -> TrieIndex {
        if ac == 0 {
            return TRIE_CHAR_TERM as TrieIndex;
        }

        if (self.alpha_begin..=self.alpha_end).contains(&ac) {
            return self
                .alpha_to_trie_map
                .get((ac - self.alpha_begin) as usize)
                .copied()
                .unwrap_or(TRIE_INDEX_MAX);
        }

        TRIE_INDEX_MAX
    }

    pub(crate) fn trie_to_char(&self, tc: TrieChar) -> AlphaChar {
        self.trie_to_alpha_map
            .get(tc as usize)
            .copied()
            .unwrap_or(ALPHA_CHAR_ERROR)
    }
}

impl Default for AlphaMap {
    fn default() -> Self {
        AlphaMap {
            first_range: ptr::null_mut(),
            alpha_begin: 0,
            alpha_end: 0,
            alpha_to_trie_map: Box::new([]),
            trie_to_alpha_map: Box::new([]),
        }
    }
}

impl Clone for AlphaMap {
    fn clone(&self) -> Self {
        let mut am = Self::default();

        if let Some(iter) = self.range_iter() {
            for range in iter {
                unsafe {
                    if alpha_map_add_range_only((&mut am).into(), range.begin, range.end) != 0 {
                        panic!("clone fail")
                    }
                }
            }
        }

        am.recalc_work_area();

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
        }
    }
}

#[deprecated(note = "Use AlphaMap::default()")]
#[no_mangle]
pub extern "C" fn alpha_map_new() -> *mut AlphaMap {
    Box::into_raw(Box::new(AlphaMap::default()))
}

#[deprecated(note = "Use a_map::clone()")]
#[no_mangle]
pub extern "C" fn alpha_map_clone(mut a_map: *const AlphaMap) -> *mut AlphaMap {
    let Some(am) = (unsafe { a_map.as_ref() }) else {
        return ptr::null_mut();
    };

    Box::into_raw(Box::new(am.clone()))
}

#[deprecated(note = "Just drop alpha_map")]
#[no_mangle]
pub unsafe extern "C" fn alpha_map_free(mut alpha_map: NonNull<AlphaMap>) {
    let am = Box::from_raw(alpha_map.as_mut());
    drop(am) // This is not strictly needed, but it helps in clarity
}

#[deprecated(note = "Use AlphaMap::read(). Careful about file position on failure!")]
#[no_mangle]
pub(crate) extern "C" fn alpha_map_fread_bin(file: NonNull<libc::FILE>) -> *mut AlphaMap {
    let mut file = wrap_cfile_nonnull(file);
    let save_pos = file.seek(SeekFrom::Current(0)).unwrap();

    match AlphaMap::read(&mut file) {
        Ok(am) => Box::into_raw(Box::new(am.clone())),
        Err(_) => {
            // Return to save_pos if read fail
            let _ = file.seek(SeekFrom::Start(save_pos));
            return ptr::null_mut();
        }
    }
}

#[deprecated(note = "Use alpha_map.serialize()")]
#[no_mangle]
pub(crate) extern "C" fn alpha_map_fwrite_bin(
    alpha_map: *const AlphaMap,
    file: NonNull<libc::FILE>,
) -> i32 {
    let mut file = wrap_cfile_nonnull(file);

    let am = unsafe { &*alpha_map };

    match am.serialize(&mut file) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[deprecated(note = "Use alpha_map.serialized_size()")]
#[no_mangle]
pub(crate) extern "C" fn alpha_map_get_serialized_size(alpha_map: *const AlphaMap) -> usize {
    unsafe { &*alpha_map }.serialized_size()
}

#[deprecated(note = "Use alpha_map.serialize()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn alpha_map_serialize_bin(
    alpha_map: *const AlphaMap,
    mut ptr: NonNull<NonNull<[u8]>>,
) {
    let mut cursor = Cursor::new(ptr.as_mut().as_mut());
    (*alpha_map).serialize(&mut cursor).unwrap();
    // Move ptr
    ptr.write(ptr.as_ref().byte_offset(cursor.position() as isize));
}

unsafe extern "C" fn alpha_map_add_range_only(
    mut alpha_map: NonNull<AlphaMap>,
    begin: AlphaChar,
    end: AlphaChar,
) -> i32 {
    if begin > end {
        return -1;
    }
    let mut begin_node = 0 as *mut AlphaRange;
    let mut end_node = 0 as *mut AlphaRange;
    let mut q = 0 as *mut AlphaRange;
    let mut r = alpha_map.as_mut().first_range;
    // Skip first ranges till 'begin' is covered
    while !r.is_null() && (*r).begin <= begin {
        if begin <= (*r).end {
            // 'r' covers 'begin' -> take 'r' as beginning point
            begin_node = r;
            break;
        } else if (*r).end + 1 == begin {
            // 'begin' is next to 'r'-end
            // -> extend 'r'-end to cover 'begin'
            (*r).end = begin;
            begin_node = r;
            break;
        }

        q = r;
        r = (*r).next;
    }
    if begin_node.is_null() && !r.is_null() && (*r).begin <= end + 1 {
        // ['begin', 'end'] overlaps into 'r'-begin
        // or 'r' is next to 'end' if r->begin == end + 1
        // -> extend 'r'-begin to include the range
        (*r).begin = begin;
        begin_node = r;
    }
    // Run upto the first range that exceeds 'end'
    while !r.is_null() && (*r).begin <= end + 1 {
        if end <= (*r).end {
            // 'r' covers 'end' -> take 'r' as ending point
            end_node = r;
        } else if r != begin_node {
            // ['begin', 'end'] covers the whole 'r' -> remove 'r'
            if !q.is_null() {
                (*q).next = (*r).next;
                free(r as *mut libc::c_void);
                r = (*q).next;
            } else {
                alpha_map.as_mut().first_range = (*r).next;
                free(r as *mut libc::c_void);
                r = alpha_map.as_ref().first_range;
            }
            continue;
        }
        q = r;
        r = (*r).next;
    }
    if end_node.is_null() && !q.is_null() && begin <= (*q).end {
        // ['begin', 'end'] overlaps 'q' at the end
        // -> extend 'q'-end to include the range
        (*q).end = end;
        end_node = q;
    }
    if !begin_node.is_null() && !end_node.is_null() {
        if begin_node != end_node {
            // Merge begin_node and end_node ranges together
            assert_eq!((*begin_node).next, end_node);
            (*begin_node).end = (*end_node).end;
            (*begin_node).next = (*end_node).next;
            free(end_node as *mut libc::c_void);
        }
    } else if begin_node.is_null() && end_node.is_null() {
        // ['begin', 'end'] overlaps with none of the ranges
        // -> insert a new range
        let mut range: *mut AlphaRange =
            malloc(size_of::<AlphaRange>() as libc::c_ulong) as *mut AlphaRange;

        if range.is_null() {
            return -1;
        }

        (*range).begin = begin;
        (*range).end = end;

        // insert it between 'q' and 'r'
        if !q.is_null() {
            (*q).next = range;
        } else {
            alpha_map.as_mut().first_range = range;
        }
        (*range).next = r;
    }

    0
}

#[no_mangle]
pub extern "C" fn alpha_map_add_range(
    mut alpha_map: NonNull<AlphaMap>,
    begin: AlphaChar,
    end: AlphaChar,
) -> i32 {
    let res = unsafe { alpha_map_add_range_only(alpha_map, begin, end) };
    if res != 0 {
        return res;
    }
    unsafe { alpha_map.as_mut() }.recalc_work_area();

    0
}

#[deprecated(note = "Use alpha_map.char_to_trie()")]
#[no_mangle]
pub(crate) extern "C" fn alpha_map_char_to_trie(
    alpha_map: *const AlphaMap,
    ac: AlphaChar,
) -> TrieIndex {
    if ac == 0 {
        return TRIE_CHAR_TERM as TrieIndex;
    }

    (unsafe { &*alpha_map }).char_to_trie(ac)
}

#[deprecated(note = "Use alpha_map.trie_to_char()")]
#[no_mangle]
pub(crate) extern "C" fn alpha_map_trie_to_char(
    alpha_map: *const AlphaMap,
    tc: TrieChar,
) -> AlphaChar {
    (unsafe { &*alpha_map }).trie_to_char(tc)
}

#[no_mangle]
pub(crate) extern "C" fn alpha_map_char_to_trie_str(
    alpha_map: *const AlphaMap,
    mut str: *const AlphaChar,
) -> *mut TrieChar {
    let str = unsafe { slice::from_raw_parts(str, alpha_char_strlen(str) as usize) };
    let am = unsafe { &*alpha_map };

    let out_vec: Option<Vec<TrieChar>> = str
        .iter()
        .map(|v| {
            let tc = am.char_to_trie(*v);
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
) -> NonNull<AlphaChar> {
    let str = unsafe { slice::from_raw_parts(str, trie_char_strlen(str)) };
    let am = unsafe { &*alpha_map };

    let out: Vec<AlphaChar> = str
        .iter()
        .map(|chr| am.trie_to_char(*chr))
        .chain(iter::once(0))
        .collect();

    unsafe { NonNull::new_unchecked(Box::into_raw(out.into_boxed_slice()).cast()) }
}

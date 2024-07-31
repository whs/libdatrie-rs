use std::{io, iter, ptr, slice};
use std::io::{Read, Write};
use std::ops::RangeInclusive;
use std::ptr::NonNull;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rangemap::RangeInclusiveSet;

use crate::trie_string::{trie_char_as_slice, TRIE_CHAR_TERM, TrieChar};
use crate::types::*;

#[derive(Clone, Default)]
pub struct AlphaMap {
    alpha_begin: AlphaChar,
    alpha_end: AlphaChar,
    ranges: RangeInclusiveSet<AlphaChar>,
    alpha_to_trie_map: Box<[TrieIndex]>,
    trie_to_alpha_map: Box<[AlphaChar]>,
}

const ALPHAMAP_SIGNATURE: u32 = 0xd9fcd9fc;

impl AlphaMap {
    pub fn add_range(&mut self, range: RangeInclusive<AlphaChar>) {
        self.ranges.insert(range);
        self.recalc_work_area()
    }

    pub(crate) fn read<T: Read>(stream: &mut T) -> io::Result<Self> {
        // check signature
        if stream.read_u32::<BigEndian>()? != ALPHAMAP_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid signature",
            ));
        }

        let mut alphamap = Self::default();

        // Read number of ranges
        let total = stream.read_i32::<BigEndian>()?;

        // Read character ranges
        for _ in 0..total {
            let begin = stream.read_i32::<BigEndian>()? as AlphaChar;
            let end = stream.read_i32::<BigEndian>()? as AlphaChar;
            if begin > end {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid range"));
            }
            alphamap.ranges.insert(begin..=end);
        }

        // work area
        alphamap.recalc_work_area();

        Ok(alphamap)
    }

    pub(crate) fn serialize<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_u32::<BigEndian>(ALPHAMAP_SIGNATURE)?;
        buf.write_i32::<BigEndian>(self.ranges.len() as i32)?;

        for range in self.ranges.iter() {
            buf.write_i32::<BigEndian>(*range.start() as i32)?;
            buf.write_i32::<BigEndian>(*range.end() as i32)?;
        }

        Ok(())
    }

    pub(crate) fn serialized_size(&self) -> usize {
        return 4 // ALPHAMAP_SIGNATURE
            + size_of::<i32>() // ranges_count
            + (size_of::<AlphaChar>() * 2 * self.ranges.len());
    }

    fn recalc_work_area(&mut self) {
        // free old existing map
        self.alpha_to_trie_map = Box::new([]);
        self.trie_to_alpha_map = Box::new([]);

        let Some(alpha_first) = self.ranges.first() else {
            return;
        };
        let alpha_begin = *alpha_first.start();

        self.alpha_begin = alpha_begin;
        // Count the total member within all self.ranges ranges
        let mut n_trie: usize = self
            .ranges
            .iter()
            .map(|range| *range.end() as usize - *range.start() as usize + 1)
            .sum();
        if n_trie < TRIE_CHAR_TERM as usize {
            // does this even hit? overflow handling?
            n_trie = TRIE_CHAR_TERM as usize + 1;
        } else {
            n_trie += 1;
        }
        self.alpha_end = *self.ranges.last().unwrap().end();

        let n_alpha = self.alpha_end as usize - alpha_begin as usize + 1;

        let mut alpha_to_trie_map = vec![TRIE_INDEX_MAX; n_alpha].into_boxed_slice();
        let mut trie_to_alpha_map = vec![ALPHA_CHAR_ERROR; n_trie].into_boxed_slice();

        let mut trie_char: TrieIndex = 0;
        for range in self.ranges.iter() {
            for a in range.clone() {
                if trie_char == TRIE_CHAR_TERM as TrieIndex {
                    trie_char += 1;
                }
                // Elide bond checks
                unsafe {
                    *alpha_to_trie_map.get_unchecked_mut((a - alpha_begin) as usize) =
                        trie_char as TrieIndex;
                    *trie_to_alpha_map.get_unchecked_mut(trie_char as usize) = a;
                }
                trie_char += 1;
            }
        }
        trie_to_alpha_map[TRIE_CHAR_TERM as usize] = 0;

        self.alpha_to_trie_map = alpha_to_trie_map;
        self.trie_to_alpha_map = trie_to_alpha_map;
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

#[deprecated(note = "Use AlphaMap::default()")]
#[no_mangle]
pub extern "C" fn alpha_map_new() -> *mut AlphaMap {
    Box::into_raw(Box::new(AlphaMap::default()))
}

#[deprecated(note = "Use a_map::clone()")]
#[no_mangle]
pub extern "C" fn alpha_map_clone(a_map: *const AlphaMap) -> *mut AlphaMap {
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

#[deprecated(note = "Use alpha_map.add_range(begin..=end)")]
#[no_mangle]
pub extern "C" fn alpha_map_add_range(
    mut alpha_map: NonNull<AlphaMap>,
    begin: AlphaChar,
    end: AlphaChar,
) -> i32 {
    if begin > end {
        return -1;
    }
    let am = unsafe { alpha_map.as_mut() };
    am.add_range(begin..=end);
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
    str: *const AlphaChar,
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
    let str = trie_char_as_slice(str);
    let am = unsafe { &*alpha_map };

    let out: Vec<AlphaChar> = str
        .iter()
        .map(|chr| am.trie_to_char(*chr))
        .chain(iter::once(0))
        .collect();

    unsafe { NonNull::new_unchecked(Box::into_raw(out.into_boxed_slice()).cast()) }
}

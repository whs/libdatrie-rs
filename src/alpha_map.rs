use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::iter;
use core::ops::RangeInclusive;
#[cfg(feature = "std")]
use std::io;
#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(feature = "std")]
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rangemap::RangeInclusiveSet;

use crate::types::*;
use crate::types::{TrieChar, TRIE_CHAR_TERM};

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

    #[cfg(feature = "std")]
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
            let range = begin..=end;
            if range.clone().count() >= u8::MAX as usize {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "range too large",
                ));
            }
            if range.clone().contains(&ALPHA_CHAR_ERROR) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "range include ALPHA_CHAR_ERROR",
                ));
            }
            alphamap.ranges.insert(range);
        }

        // work area
        alphamap.recalc_work_area();

        Ok(alphamap)
    }

    #[cfg(feature = "std")]
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
                alpha_to_trie_map[(a - alpha_begin) as usize] = trie_char as TrieIndex;
                trie_to_alpha_map[trie_char as usize] = a;
                trie_char += 1;
            }
        }
        trie_to_alpha_map[TRIE_CHAR_TERM as usize] = 0;

        self.alpha_to_trie_map = alpha_to_trie_map;
        self.trie_to_alpha_map = trie_to_alpha_map;
    }

    pub(crate) fn char_to_trie(&self, ac: AlphaChar) -> Option<TrieIndex> {
        if ac == 0 {
            return Some(TRIE_CHAR_TERM as TrieIndex);
        }

        if (self.alpha_begin..=self.alpha_end).contains(&ac) {
            return self
                .alpha_to_trie_map
                .get((ac - self.alpha_begin) as usize)
                .copied();
        }

        None
    }

    pub(crate) fn char_to_trie_str(&self, str: &[AlphaChar]) -> Option<Vec<TrieChar>> {
        str.iter()
            .copied()
            .map_to_trie_char(self)
            .chain(iter::once(Some(TRIE_CHAR_TERM)))
            .collect()
    }

    pub(crate) fn trie_to_char(&self, tc: TrieChar) -> AlphaChar {
        self.trie_to_alpha_map
            .get(tc as usize)
            .copied()
            .unwrap_or(ALPHA_CHAR_ERROR)
    }
}

pub trait ToAlphaChars {
    fn map_to_alpha_char(self, alpha_map: &AlphaMap) -> impl Iterator<Item = AlphaChar>;
}

impl<T: Iterator<Item = TrieChar>> ToAlphaChars for T {
    fn map_to_alpha_char(self, alpha_map: &AlphaMap) -> impl Iterator<Item = AlphaChar>
    where
        Self: Sized,
    {
        self.map_while(|chr| match chr {
            TRIE_CHAR_TERM => None,
            chr => Some(alpha_map.trie_to_char(chr)),
        })
    }
}

pub trait ToTrieChar {
    fn map_to_trie_char(self, alpha_map: &AlphaMap) -> impl Iterator<Item = Option<TrieChar>>;
}

impl<T: Iterator<Item = AlphaChar>> ToTrieChar for T {
    fn map_to_trie_char(self, alpha_map: &AlphaMap) -> impl Iterator<Item = Option<TrieChar>>
    where
        Self: Sized,
    {
        self.map(|chr| alpha_map.char_to_trie(chr).map(|v| v as TrieChar))
    }
}

#[cfg(feature = "cffi")]
mod cffi {
    use crate::alpha_map::*;
    use std::ptr;
    use std::ptr::NonNull;

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

    #[no_mangle]
    pub unsafe extern "C" fn alpha_map_free(mut alpha_map: NonNull<AlphaMap>) {
        drop(Box::from_raw(alpha_map.as_mut()))
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
}

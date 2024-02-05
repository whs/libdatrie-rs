////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

use crate::alphamaploader::AlphaMapLoader;
/**
 * AlphaMap is a mapping between AlphaChar and TrieChar. AlphaChar is the
 * alphabet character used in words of a target language, while TrieChar
 * is a small integer with packed range of values and is actually used in
 * trie state transition calculations.
 *
 * Since double-array trie relies on sparse state transition table,
 * a small set of input characters can make the table small, i.e. with
 * small number of columns. But in real life, alphabet characters can be
 * of non-continuous range of values. The unused slots between them can
 * waste the space in the table, and can increase the chance of unused
 * array cells.
 *
 * AlphaMap is thus defined for mapping between non-continuous ranges of
 * values of AlphaChar and packed and continuous range of Triechar.
 *
 * In this implementation, TrieChar is defined as a single-byte integer,
 * which means the largest AlphaChar set that is supported is of 255
 * values, as the special value of 0 is reserved for null-termination code.
 */
use crate::{AlphaChar, TrieChar, TrieIndex};
use byteorder::{BigEndian, WriteBytesExt};
use range_map::{Range, RangeSet};
use std::io;
use std::io::{Read, Write};

pub(super) const ALPHAMAP_SIGNATURE: u32 = 0xD9FCD9FC;

#[derive(Clone, Debug)]
pub struct AlphaMap {
    set: RangeSet<AlphaChar>,

    // computed fields
    min: AlphaChar,
    alpha2trie: Vec<Option<TrieIndex>>,
    trie2alpha: Vec<Option<AlphaChar>>,
}

impl AlphaMap {
    pub fn new() -> Self {
        Self {
            set: RangeSet::new(),

            min: Default::default(),
            alpha2trie: Vec::new(),
            trie2alpha: Vec::new(),
        }
    }

    pub fn add_range(&mut self, start: AlphaChar, end: AlphaChar) {
        self.set = self.set.union(&RangeSet::from_iter([Range { start, end }]));
        self.rebuild();
    }

    fn rebuild(&mut self) {
        // alpha_begin = first character = min

        let first_range = match self.set.ranges().next() {
            Some(v) => v,
            None => {
                self.alpha2trie = Vec::new();
                self.trie2alpha = Vec::new();
                return;
            }
        };
        let min = first_range.start;
        let max = self.set.ranges().last().unwrap().end;
        self.min = min;
        let n_trie = self.set.num_elements() + 1;
        let n_alpha = max - min + 1;

        self.alpha2trie = vec![None; n_alpha as usize];
        self.trie2alpha = vec![None; n_trie];

        for (index, value) in (1..).zip(self.set.elements()) {
            self.alpha2trie[(value - min) as usize] = Some(index as TrieIndex);
            self.trie2alpha[index] = Some(value);
        }

        self.trie2alpha[0] = Some(0);
    }

    pub fn char_to_trie(&self, ch: AlphaChar) -> Option<TrieIndex> {
        if ch == 0 {
            return Some(0);
        }
        if ch < self.min {
            return None;
        }

        // TODO flatten: rust#67441
        match self.alpha2trie.get((ch - self.min) as usize) {
            Some(v) => *v,
            None => None,
        }
    }

    pub fn to_trie_str(&self, str: &[AlphaChar]) -> Option<Vec<TrieChar>> {
        let mut error = false;
        // TOOD try_collect: rust#94047
        let out = str
            .iter()
            .copied()
            .map_while(|ch| {
                let trie_ch = self.char_to_trie(ch);
                if trie_ch.is_none() {
                    error = true;
                }
                trie_ch.map(|v| v as TrieChar)
            })
            .collect();

        if error {
            return None;
        }

        Some(out)
    }

    pub fn to_char(&self, ch: TrieChar) -> Option<AlphaChar> {
        // TODO flatten: rust#67441
        match self.trie2alpha.get(ch as usize) {
            Some(v) => *v,
            None => None,
        }
    }

    pub fn to_tries<'a, T: 'a + Iterator<Item = AlphaChar>>(
        &'a self,
        ch: T,
    ) -> impl Iterator<Item = Option<TrieIndex>> + 'a {
        ch.map(|v| self.char_to_trie(v))
    }

    /// Map input AlphaChar iterator to TrieChar, dropping invalid characters
    pub fn to_tries_without_invalids<'a, T: 'a + Iterator<Item = AlphaChar>>(
        &'a self,
        ch: T,
    ) -> impl Iterator<Item = TrieIndex> + 'a {
        ch.map(|v| self.char_to_trie(v)).filter_map(|v| v)
    }

    pub fn to_alphas<'a, T: 'a + Iterator<Item = TrieChar>>(
        &'a self,
        ch: T,
    ) -> impl Iterator<Item = Option<AlphaChar>> + 'a {
        ch.map(|v| self.to_char(v))
    }

    /// Map input TrieChar iterator to AlphaChar, dropping invalid characters
    pub fn to_alphas_without_invalids<'a, T: 'a + Iterator<Item = TrieChar>>(
        &'a self,
        ch: T,
    ) -> impl Iterator<Item = AlphaChar> + 'a {
        ch.map(|v| self.to_char(v)).filter_map(|v| v)
    }

    pub fn load<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let loader = AlphaMapLoader::new(reader)?;
        let loader_res = loader.collect::<io::Result<Vec<_>>>()?;
        let set = RangeSet::from_iter(loader_res.into_iter().map(|item| Range {
            start: item.0.into(),
            end: item.1.into(),
        }));
        self.set = set;
        self.rebuild();
        Ok(())
    }

    pub fn save<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<BigEndian>(ALPHAMAP_SIGNATURE)?;
        writer.write_i32::<BigEndian>(self.set.num_ranges() as i32)?;

        for range in self.set.ranges() {
            writer.write_i32::<BigEndian>(range.start as i32)?;
            writer.write_i32::<BigEndian>(range.end as i32)?;
        }

        Ok(())
    }
}

impl Default for AlphaMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::alphamap::AlphaMap;

    #[test]
    fn to_trie() {
        let mut map = AlphaMap::new();
        map.add_range(10, 20);
        map.add_range(100, 110);

        assert_eq!(map.char_to_trie(0), Some(0));
        assert_eq!(map.char_to_trie(10), Some(1));
        assert_eq!(map.char_to_trie(20), Some(11));
        assert_eq!(map.char_to_trie(21), None);
        assert_eq!(map.char_to_trie(100), Some(12));
        assert_eq!(map.char_to_trie(110), Some(22));
        assert_eq!(map.char_to_trie(111), None);
        assert_eq!(map.char_to_trie(10000), None);
    }

    #[test]
    fn map_255() {
        let mut map = AlphaMap::new();
        map.add_range(0, 255);
        assert_eq!(map.char_to_trie(0), Some(0));
        assert_eq!(map.char_to_trie(254), Some(255));
        assert_eq!(map.char_to_trie(255), Some(256));
        assert_eq!(map.to_char(255), Some(254));
    }

    #[test]
    fn to_char() {
        let mut map = AlphaMap::new();
        map.add_range(10, 20);
        map.add_range(100, 110);

        assert_eq!(map.to_char(0), Some(0));
        assert_eq!(map.to_char(10), Some(19));
        assert_eq!(map.to_char(11), Some(20));
        assert_eq!(map.to_char(12), Some(100));
        assert_eq!(map.to_char(22), Some(110));
        assert_eq!(map.to_char(23), None);
        assert_eq!(map.to_char(255), None);
    }

    #[test]
    fn to_tries() {
        let mut map = AlphaMap::new();
        map.add_range(15, 19);
        map.add_range(10, 20);
        map.add_range(100, 110);

        assert_eq!(
            map.to_tries([0, 10, 20, 21, 100, 110, 111, 10000].into_iter())
                .collect::<Vec<_>>(),
            [
                Some(0),
                Some(1),
                Some(11),
                None,
                Some(12),
                Some(22),
                None,
                None,
            ]
        );
    }

    #[test]
    fn to_alphas() {
        let mut map = AlphaMap::new();
        map.add_range(15, 19);
        map.add_range(10, 20);
        map.add_range(100, 110);

        assert_eq!(
            map.to_alphas([0, 1, 10, 11, 12, 21, 22, 23, 255].into_iter())
                .collect::<Vec<_>>(),
            [
                Some(0),
                Some(10),
                Some(19),
                Some(20),
                Some(100),
                Some(109),
                Some(110),
                None,
                None
            ]
        );
    }
}

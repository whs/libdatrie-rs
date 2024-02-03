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

use crate::tailloader::TailLoader;
use crate::{TrieChar, TrieData, TrieIndex};
use byteorder::{BigEndian, WriteBytesExt};
use std::cmp::min;
use std::io;
use std::io::{Read, Write};
use std::iter::zip;

pub(super) const TAIL_SIGNATURE: u32 = 0xDFFCDFFC;
const TAIL_START_BLOCKNO: TrieIndex = 1;

#[derive(Default, Clone, Debug)]
pub struct Tail {
    first_free: i32,
    tails: Vec<TailData>,
}

impl Tail {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let loader = TailLoader::new(reader)?;
        self.first_free = loader.first_free;
        self.tails = loader.collect::<io::Result<Vec<TailData>>>()?;
        Ok(())
    }

    pub fn save<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<BigEndian>(TAIL_SIGNATURE)?;
        writer.write_i32::<BigEndian>(self.first_free)?;
        writer.write_i32::<BigEndian>(self.tails.len() as i32)?;

        for tail in &self.tails {
            let next_free: i32 = match tail.next_free {
                None => -1,
                Some(v) => v,
            };
            writer.write_i32::<BigEndian>(next_free)?;
            writer.write_i32::<BigEndian>(tail.data.unwrap_or(-1))?;
            writer.write_i16::<BigEndian>(tail.suffix.len() as i16)?;
            writer.write(&tail.suffix)?;
        }

        Ok(())
    }

    pub fn get_suffix(&self, index: TrieIndex) -> Option<&Vec<TrieChar>> {
        let idx = index - TAIL_START_BLOCKNO;
        Some(&self.tails.get(idx as usize)?.suffix)
    }

    pub fn set_suffix(&mut self, index: TrieIndex, suffix: &[TrieChar]) -> Result<(), ()> {
        let idx = index - TAIL_START_BLOCKNO;
        let mut item = self.tails.get_mut(idx as usize).ok_or(())?;
        item.suffix = Vec::from(suffix);
        Ok(())
    }

    pub fn add_suffix(&mut self, suffix: &[TrieChar]) -> TrieIndex {
        let new_block = self.alloc_block();
        self.set_suffix(new_block, suffix).unwrap();
        new_block
    }

    fn alloc_block(&mut self) -> TrieIndex {
        if self.first_free != 0 {
            let block_id = self.first_free;
            self.first_free = self.tails[block_id as usize].next_free.unwrap();

            let block = self.tails.get_mut(block_id as usize).unwrap();
            block.next_free = None;
            block.data = None;
            block.suffix = Vec::new();

            block_id + TAIL_START_BLOCKNO
        } else {
            self.tails.push(TailData::default());

            (self.tails.len() as TrieIndex) - 1 + TAIL_START_BLOCKNO
        }
    }

    pub fn get_data(&self, index: TrieIndex) -> Option<TrieData> {
        let idx = index - TAIL_START_BLOCKNO;
        self.tails.get(idx as usize)?.data
    }

    pub fn set_data(&mut self, index: TrieIndex, data: TrieData) -> Result<(), ()> {
        let idx = index - TAIL_START_BLOCKNO;
        let mut item = self.tails.get_mut(idx as usize).ok_or(())?;
        item.data = Some(data);
        Ok(())
    }

    /// Walk in the tail data at index, from given character position
    /// suffix_idx, using a given string.
    /// Returns the total numbers of characters successfully walked.
    ///
    /// For compatiblity with C libdatrie, advance suffix_idx by the returned value.
    #[must_use]
    pub fn walk_str(&self, index: TrieIndex, suffix_idx: usize, str: &[TrieChar]) -> usize {
        let suffix = match self.get_suffix(index) {
            Some(v) => v,
            None => return 0,
        };

        let iter = zip(str.iter(), suffix[suffix_idx..].iter());

        for (index, (in_ch, suffix_ch)) in iter.enumerate() {
            if in_ch != suffix_ch {
                return index;
            }
        }

        min(str.len(), suffix.len())
    }

    /// Walk in the tail data at index, from given character position
    /// suffix_idx, using given character c. If the walk is successful,
    /// it returns the next character index.
    /// Otherwise, it returns None
    #[must_use]
    pub fn walk_char(&self, index: TrieIndex, suffix_idx: usize, char: TrieChar) -> Option<usize> {
        let suffix = match self.get_suffix(index) {
            Some(v) => v,
            None => return None,
        };
        let suffix_char = suffix.get(suffix_idx).copied();
        if suffix_char == Some(char) {
            return Some(min(suffix_idx + 1, suffix.len()));
        }
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct TailData {
    pub(super) next_free: Option<TrieIndex>,
    pub(super) data: Option<TrieData>,
    pub(super) suffix: Vec<TrieChar>,
}

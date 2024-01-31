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
use std::io;
use std::io::{Read, Write};

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
        writer.write_i32::<BigEndian>(self.first_free as i32)?;
        writer.write_i32::<BigEndian>(self.tails.len() as i32)?;

        for tail in &self.tails {
            let next_free: i32 = match tail.next_free {
                None => -1,
                Some(v) => v,
            };
            writer.write_i32::<BigEndian>(next_free)?;
            writer.write_i32::<BigEndian>(tail.data)?;
            writer.write_i16::<BigEndian>(tail.suffix.len() as i16)?;
            writer.write(&tail.suffix)?;
        }

        Ok(())
    }

    pub fn get_suffix(&self, index: TrieIndex) -> Option<&Vec<u8>> {
        let idx = index - TAIL_START_BLOCKNO;
        Some(&self.tails.get(idx as usize)?.suffix)
    }

    pub fn set_suffix(&mut self, index: TrieIndex, suffix: &Vec<u8>) -> Result<(), ()> {
        let idx = index - TAIL_START_BLOCKNO;
        let mut item = self.tails.get_mut(idx as usize).ok_or(())?;
        item.suffix = suffix.clone();
        Ok(())
    }

    pub fn get_data(&self, index: TrieIndex) -> Option<TrieData> {
        let idx = index - TAIL_START_BLOCKNO;
        Some(self.tails.get(idx as usize)?.data)
    }

    pub fn set_data(&mut self, index: TrieIndex, data: TrieData) -> Result<(), ()> {
        let idx = index - TAIL_START_BLOCKNO;
        let mut item = self.tails.get_mut(idx as usize).ok_or(())?;
        item.data = data;
        Ok(())
    }

    pub fn walk_char(&self, index: TrieIndex, suffix_idx: usize, char: TrieChar) -> bool {
        let suffix = match self.get_suffix(index) {
            Some(v) => v,
            None => return false,
        };
        let suffix_char = suffix.get(suffix_idx).copied();
        if suffix_char == Some(char) {
            return true;
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct TailData {
    pub(super) next_free: Option<TrieIndex>,
    pub(super) data: TrieData,
    pub(super) suffix: Vec<TrieChar>,
}

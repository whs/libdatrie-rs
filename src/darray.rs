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

use crate::darrayloader::DarrayLoader;
use crate::{TrieChar, TrieIndex};
use byteorder::{BigEndian, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

pub(super) const DA_SIGNATURE: u32 = 0xDAFCDAFC;

#[derive(Clone, Debug)]
pub struct Cell {
    pub base: TrieIndex,
    pub check: TrieIndex,
}

#[derive(Default, Clone, Debug)]
pub struct DArray {
    cell: Vec<Cell>,
}

impl DArray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_root(&self) -> TrieIndex {
        2
    }

    pub fn get_base(&self, index: TrieIndex) -> Option<TrieIndex> {
        self.cell.get(index as usize).map(|v| v.base)
    }

    pub fn get_check(&self, index: TrieIndex) -> Option<TrieIndex> {
        self.cell.get(index as usize).map(|v| v.check)
    }

    pub fn set_base(&mut self, index: TrieIndex, value: TrieIndex) {
        match self.cell.get_mut(index as usize) {
            Some(v) => v.base = value,
            None => {}
        }
    }

    pub fn set_check(&mut self, index: TrieIndex, value: TrieIndex) {
        match self.cell.get_mut(index as usize) {
            Some(v) => v.check = value,
            None => {}
        }
    }

    /// Walk the double-array trie from state index
    /// If there exists an edge from index with arc labeled input_char, this function
    /// returns the new state. Otherwise, it returns None
    #[must_use]
    pub fn walk(&self, index: TrieIndex, input_char: TrieChar) -> Option<TrieIndex> {
        let next = self.get_base(index)? + input_char as i32;
        if self.get_check(next) == Some(index) {
            return Some(next);
        }
        None
    }

    #[inline]
    pub fn is_walkable(&self, state: TrieIndex, input_char: TrieChar) -> Option<bool> {
        // da_get_check (
        //      (d),
        //      da_get_base(d, s) + c
        // ) == (s)
        let base = self.get_base(state)? + input_char as i32;
        Some(self.get_check(base) == Some(state))
    }

    #[inline]
    pub fn is_separate(&self, s: TrieIndex) -> bool {
        match self.get_base(s) {
            Some(v) => v < 0,
            None => false,
        }
    }

    #[inline]
    pub fn get_tail_index(&self, s: TrieIndex) -> Option<TrieIndex> {
        Some(-self.get_base(s)?)
    }

    #[inline]
    pub(crate) fn set_tail_index(&mut self, s: TrieIndex, v: TrieIndex) {
        self.set_base(s, -v)
    }

    /// Find first separate node in a sub-trie
    ///
    /// Find the first separate node under a sub-trie rooted at root.
    /// On return, keybuff is appended with the key characters which walk from
    /// root to the separate node. This is for incrementally calculating the
    /// transition key, which is more efficient than later totally reconstructing
    /// key from the given separate node.
    pub fn first_separate(
        &self,
        root: TrieIndex,
        keybuff: &mut Vec<TrieChar>,
    ) -> Option<TrieIndex> {
        let mut cur = root;
        loop {
            let base = match self.get_base(cur) {
                Some(v) if v < 0 => return Some(cur),
                Some(v) => v,
                None => 0,
            };

            let max_c_cells = self.cell.len() as TrieIndex - base;
            let max_c = (TrieChar::MAX as TrieIndex).min(max_c_cells);
            // that libdatrie code is too clever..
            let c = (0..=max_c).find(|c| self.get_check(base + c) == Some(cur));
            match c {
                None => return None,
                Some(c) => {
                    keybuff.push(c as TrieChar);
                    cur = base + c;
                }
            }
        }
    }

    pub fn next_separate(
        &self,
        root: TrieIndex,
        sep: TrieIndex,
        keybuff: &mut Vec<TrieChar>,
    ) -> Option<TrieIndex> {
        let mut cur = sep;

        while cur != root {
            let parent = self.get_check(cur)?;
            let base = self.get_base(parent)?;
            keybuff.pop();

            let start_c = (cur - base) as TrieIndex;
            let max_c = (TrieChar::MAX as TrieIndex).min(self.cell.len() as i32 - base);

            for c in (start_c + 1)..=max_c {
                if self.get_check(base + c) == Some(parent) {
                    keybuff.push(c as TrieChar);
                    return self.first_separate(base + c, keybuff);
                }
            }

            cur = parent;
        }

        None
    }

    pub fn load<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let loader = DarrayLoader::new(reader)?;
        self.cell = loader.collect::<io::Result<Vec<Cell>>>()?;
        Ok(())
    }

    pub fn save<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<BigEndian>(DA_SIGNATURE)?;
        writer.write_i32::<BigEndian>(self.cell.len() as i32)?;

        for cell in &self.cell {
            writer.write_i32::<BigEndian>(cell.base)?;
            writer.write_i32::<BigEndian>(cell.check)?;
        }

        Ok(())
    }
}

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
use crate::symbols::Symbols;
use crate::{TrieChar, TrieIndex};
use byteorder::{BigEndian, WriteBytesExt};
use std::cmp::min;
use std::io;
use std::io::{Read, Write};
use std::mem::size_of;

pub(crate) const DA_SIGNATURE: u32 = 0xDAFCDAFC;

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
        let mut cell = vec![
            // Signature
            Cell {
                base: DA_SIGNATURE as TrieIndex,
                check: 3,
            },
            // Free circular list pointers
            Cell {
                base: -1,
                check: -1,
            },
            // Root node
            Cell { base: Self::POOL_BEGIN, check: 0 },
        ];

        DArray { cell }
    }

    pub fn get_root(&self) -> TrieIndex {
        // Position of root node according to new()
        2
    }

    #[inline]
    fn get_free_list(&self) -> TrieIndex {
        // Position of free circular list according to new()
        1
    }

    const POOL_BEGIN: TrieIndex = 3;

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
        let next = self.get_base(index).unwrap() + input_char as TrieIndex;
        if self.get_check(next) == Some(index) {
            return Some(next);
        }
        None
    }

    /// Insert a new arc labelled with character char from the trie node index
    /// Note that it assumes that no such arc exists before inserting.
    /// Return the index of the new node
    pub fn insert_branch(&mut self, index: TrieIndex, char: TrieChar) -> Option<TrieIndex> {
        let base = self.get_base(index);
        let insert_point = match base {
            Some(base) => {
                let next = base.checked_add(char as TrieIndex);

                match next {
                    Some(next) if self.get_check(next) == Some(index) => {
                        // If already there, do not actually insert
                        return Some(next);
                    }
                    Some(next) if self.check_free_cell(next) => next,
                    _ => {
                        // Relocate if overflow or not free
                        let mut symbols = self.output_symbols(index);
                        symbols.add(char);
                        let new_base = self.find_free_base(&symbols)?;

                        self.relocate_base(index, new_base);

                        new_base + (char as TrieIndex)
                    }
                }
            }
            None => {
                let mut symbols = Symbols::new();
                symbols.add(char);
                let new_base = self.find_free_base(&symbols)?;
                self.set_base(index, new_base);

                new_base + (char as TrieIndex)
            }
        };
        self.alloc_cell(insert_point);
        self.set_check(insert_point, index);
        Some(insert_point)
    }

    fn check_free_cell(&mut self, index: TrieIndex) -> bool {
        self.extend_pool(index) && self.get_check(index) == Some(-1)
    }

    fn has_children(&self, index: TrieIndex) -> bool {
        let base = match self.get_base(index) {
            Some(v) if v < 0 => return false,
            Some(v) => v,
            None => return false,
        };

        let max_c = min(
            TrieChar::MAX as TrieIndex,
            self.cell.len() as TrieIndex - base,
        );
        for c in 0..=max_c {
            if self.get_check(base + c) == Some(index) {
                return true;
            }
        }

        false
    }

    fn output_symbols(&self, index: TrieIndex) -> Symbols {
        let base = self.get_base(index).unwrap();

        let max_c = min(
            TrieChar::MAX as TrieIndex,
            self.cell.len() as TrieIndex - base,
        );

        let mut out = Symbols::new();
        for c in 0..=max_c {
            if self.get_check(base + c) == Some(index) {
                out.add_fast(c as TrieChar);
            }
        }

        out
    }

    fn find_free_base(&mut self, symbols: &Symbols) -> Option<TrieIndex> {
        // find first free cell that is beyond the first symbol
        let mut first_sym = symbols.get(0).unwrap();
        let mut s = -self.get_check(self.get_free_list()).unwrap();
        let pool_start = (first_sym as TrieIndex) + Self::POOL_BEGIN;
        while s != self.get_free_list() && s < pool_start {
            s = -self.get_check(s).unwrap();
        }

        if s == self.get_free_list() {
            s = pool_start;
            loop {
                if !self.extend_pool(s) {
                    return None;
                }
                if self.get_check(s).unwrap() < 0 {
                    break;
                }
                s += 1;
            }
        }

        // search for next free cell that fits the symbols set
        while !self.fit_symbol(s - (first_sym as TrieIndex), &symbols) {
            // extend pool before getting exhausted
            if self.get_check(s) == Some(-self.get_free_list()) {
                self.extend_pool(self.cell.len() as TrieIndex);
            }
            s = -self.get_check(s).unwrap();
        }

        Some(s - (first_sym as TrieIndex))
    }

    fn fit_symbol(&mut self, base: TrieIndex, symbols: &Symbols) -> bool {
        symbols.iter().copied().all(|sym| {
            // Symbol fit if base+sym is not overflowing, and the cell is free
            base.checked_add(sym as TrieIndex) != None
                && self.check_free_cell(base + (sym as TrieIndex))
        })
    }

    fn relocate_base(&mut self, base: TrieIndex, new_base: TrieIndex) {
        let old_base = self.get_base(base).unwrap();
        let symbols = self.output_symbols(base);
        for symbol in symbols.iter().copied() {
            let old_next = old_base + (symbol as TrieIndex);
            let new_next = new_base + (symbol as TrieIndex);
            let old_next_base = self.get_base(old_next).unwrap();

            // allocate new next node and copy BASE value
            self.alloc_cell(new_next);
            self.set_check(new_next, base);
            self.set_base(new_next, old_next_base);

            // old_next node is now moved to new_next
            // so, all cells belonging to old_next must be given to new_next
            if old_next_base > 0 {
                let max_c = min(
                    TrieChar::MAX as TrieIndex,
                    self.cell.len() as TrieIndex - old_next_base,
                );
                for c in 0..=max_c {
                    if self.get_check(old_next_base + c) == Some(old_next) {
                        self.set_check(old_next_base + c, new_next)
                    }
                }
            }

            self.free_cell(old_next);
        }

        self.set_base(base, new_base);
    }

    fn extend_pool(&mut self, to_index: TrieIndex) -> bool {
        debug_assert_eq!(self.cell[0].check, self.cell.len() as TrieIndex);
        if to_index <= 0 || to_index > TrieIndex::MAX {
            return false;
        }
        if to_index < self.cell.len() as TrieIndex {
            return true;
        }

        let new_begin = self.cell.len() as TrieIndex;
        let additional = to_index + 1 - self.cell.len() as TrieIndex;
        self.cell.reserve_exact(additional as usize);

        for i in new_begin..=to_index {
            self.cell.push(Cell {
                check: -(i + 1),
                base: -(i - 1), // old code is set_base(i+1, -i)
            })
        }

        // merge the new circular list to the old
        let free_tail = -self.get_base(self.get_free_list()).unwrap();
        self.set_check(free_tail, -new_begin);
        self.set_base(new_begin, -free_tail);
        self.set_check(to_index, -self.get_free_list());
        self.set_base(self.get_free_list(), -to_index);

        // update header cell
        self.cell[0].check = self.cell.len() as TrieIndex;

        true
    }

    /// Prune off a non-separate path up from the final state.
    /// If the index still has some children states, it does nothing. Otherwise,
    /// it deletes the node and all its parents which become non-separate.
    fn prune(&mut self, index: TrieIndex) {
        self.prune_upto(self.get_root(), index)
    }

    /// Prune off a non-separate path up from the final state to the
    /// given parent. The prunning stop when either the parent
    /// is met, or a first non-separate node is found.
    fn prune_upto(&mut self, parent: TrieIndex, prune: TrieIndex) {
        let mut prune = prune;
        while prune != parent && !self.has_children(prune) {
            let prev_parent = self.get_check(prune).unwrap_or(0);
            self.free_cell(prune);
            prune = prev_parent;
        }
    }

    fn alloc_cell(&mut self, index: TrieIndex) {
        let prev = -self.get_base(index).unwrap_or(0);
        let next = -self.get_check(index).unwrap_or(0);

        self.set_check(prev, -next);
        self.set_base(next, -prev);
    }

    fn free_cell(&mut self, cell: TrieIndex) {
        // find insertion point
        let mut i = -self.get_check(self.get_free_list()).unwrap();
        while i != self.get_free_list() && i < cell {
            i = -self.get_check(i).unwrap();
        }

        let prev = -self.get_base(i).unwrap();

        // insert cell before i
        self.set_check(cell, -i);
        self.set_base(cell, -prev);
        self.set_check(prev, -cell);
        self.set_base(i, -cell);
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
            let base = self.get_base(cur).unwrap();
            if base < 0 {
                return Some(cur);
            }

            let max_c_cells = self.cell.len() as TrieIndex - base;
            let max_c = min(TrieChar::MAX as TrieIndex, max_c_cells);
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

    /// Find the next separate node under a sub-trie rooted at root starting
    /// from the current separate node sep.
    ///
    /// On return, keybuff is incrementally updated from the key which walks
    /// to previous separate node to the one which walks to the new separate node.
    /// So, it is assumed to be initialized by at least one [da_first_separate]
    /// call before. This incremental key calculation is more efficient than later
    /// totally reconstructing key from the given separate node.
    pub fn next_separate(
        &self,
        root: TrieIndex,
        sep: TrieIndex,
        keybuff: &mut Vec<TrieChar>,
    ) -> Option<TrieIndex> {
        let mut cur = sep;

        while cur != root {
            let parent = self.get_check(cur).unwrap();
            let base = self.get_base(parent).unwrap();
            keybuff.pop();

            let start_c = (cur - base + 1) as TrieIndex;
            let max_c = (TrieChar::MAX as TrieIndex).min(self.cell.len() as i32 - base);

            for c in start_c..=max_c {
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

    pub fn get_serialized_size(&self) -> usize {
        match self.cell.len() {
            0 => 0,
            // sizeof repr(C) Cell
            size => size * size_of::<TrieIndex>() * 2,
        }
    }
}

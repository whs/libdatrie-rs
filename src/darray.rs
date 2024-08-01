use std::io::{Read, Write};
use std::ptr::NonNull;
use std::{cmp, io};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::symbols::Symbols;
use crate::types::*;
use crate::types::{TRIE_CHAR_MAX, TrieChar};

#[derive(Default, Clone)]
pub(crate) struct DACell {
    base: TrieIndex,
    check: TrieIndex,
}

pub(crate) struct DArray {
    cells: Vec<DACell>,
}

const DA_SIGNATURE: u32 = 0xdafcdafc;

// DA Header:
// - Cell 0: SIGNATURE, number of cells
// - Cell 1: free circular-list pointers
// - Cell 2: root node
// - Cell 3: DA pool begin
const DA_POOL_BEGIN: TrieIndex = 3;

impl DArray {
    pub(crate) fn get_free_list(&self) -> TrieIndex {
        1
    }

    pub(crate) fn get_root(&self) -> TrieIndex {
        2
    }

    /// Get BASE cell value for the given state.
    pub(crate) fn get_base(&self, s: TrieIndex) -> Option<TrieIndex> {
        // TODO: Handle TRIE_INDEX_ERROR?
        self.cells.get(s as usize).map(|v| v.base)
    }

    /// Set BASE cell for the given state to the given value.
    pub(crate) fn set_base(&mut self, s: TrieIndex, val: TrieIndex) -> Option<()> {
        match self.cells.get_mut(s as usize) {
            Some(cell) => {
                cell.base = val;
                Some(())
            }
            None => None,
        }
    }

    /// Get CHECK cell value for the given state.
    pub(crate) fn get_check(&self, s: TrieIndex) -> Option<TrieIndex> {
        self.cells.get(s as usize).map(|v| v.check)
    }

    /// Set CHECK cell for the given state to the given value.
    pub(crate) fn set_check(&mut self, s: TrieIndex, val: TrieIndex) -> Option<()> {
        match self.cells.get_mut(s as usize) {
            Some(cell) => {
                cell.check = val;
                Some(())
            }
            None => None,
        }
    }

    /// Walk the double-array trie from state `s`, using input character `c`.
    /// If there exists an edge from `s` with arc labeled `c`, this function
    /// returns the new state. Otherwise, it returns None.
    #[must_use]
    pub(crate) fn walk(&self, s: TrieIndex, c: TrieChar) -> Option<TrieIndex> {
        // The C code doesn't handle get_base() error here
        // either it is infallible or it abuses TRIE_INDEX_ERROR
        let next = self.get_base(s).unwrap() + c as TrieIndex;
        if self.get_check(next) == Some(s) {
            return Some(next);
        }
        None
    }

    /// Insert a new arc labelled with character `c` from the trie node
    /// represented by index `s`
    /// Note that it assumes that no such arc exists before inserting.
    pub(crate) fn insert_branch(&mut self, s: TrieIndex, c: TrieChar) -> Option<TrieIndex> {
        let base = self.get_base(s).unwrap();

        let mut next;
        if base > 0 {
            next = base + c as TrieIndex;

            // if already there, do not actually insert
            if self.get_check(next) == Some(s) {
                return Some(next);
            }

            // if (base + c) > TRIE_INDEX_MAX which means 'next' is overflow,
            // or cell [next] is not free, relocate to a free slot
            if base > TRIE_INDEX_MAX - c as TrieIndex || !self.check_free_cell(next) {
                // relocate BASE[s]
                let mut symbols = self.output_symbols(s);
                symbols.add(c);
                let new_base = self.find_free_base(&symbols)?;

                self.relocate_base(s, new_base);
                next = new_base + c as TrieIndex;
            }
        } else {
            let mut symbols = Symbols::default();
            symbols.add(c);
            let new_base = self.find_free_base(&symbols)?;

            self.set_base(s, new_base);
            next = new_base + c as TrieIndex;
        }
        self.alloc_cell(next);
        self.set_check(next, s);
        Some(next)
    }

    fn check_free_cell(&mut self, s: TrieIndex) -> bool {
        if !self.extend_pool(s) {
            return false;
        }
        match self.get_check(s) {
            Some(v) if v < 0 => true,
            _ => false,
        }
    }

    fn has_children(&self, s: TrieIndex) -> bool {
        let Some(base) = self.get_base(s) else {
            return false;
        };
        if base < 0 {
            return false;
        }
        let max_c = cmp::min(
            TRIE_CHAR_MAX as TrieIndex,
            self.cells.len() as TrieIndex - base,
        );
        for c in 0..=max_c {
            if self.get_check(base + c) == Some(s) {
                return true;
            }
        }
        return false;
    }

    pub(crate) fn output_symbols(&self, s: TrieIndex) -> Symbols {
        let mut syms = Symbols::default();
        let base = self.get_base(s).unwrap();
        let max_c = cmp::min(
            TrieChar::MAX as TrieIndex,
            self.cells.len() as TrieIndex - base,
        );
        for c in 0..=max_c {
            if self.get_check(base + c) == Some(s) {
                syms.add_fast(c as TrieChar);
            }
        }
        syms
    }

    fn find_free_base(&mut self, symbols: &Symbols) -> Option<TrieIndex> {
        // find first free cell that is beyond the first symbol
        let first_sym = symbols.get(0).unwrap();
        let mut s = -self.get_check(self.get_free_list()).unwrap();
        while s != self.get_free_list() && s < first_sym as TrieIndex + DA_POOL_BEGIN {
            s = -self.get_check(s).unwrap();
        }
        if s == self.get_free_list() {
            s = first_sym as TrieIndex + DA_POOL_BEGIN;
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
        while !self.fit_symbols(s - first_sym as TrieIndex, symbols) {
            // extend pool before getting exhausted
            if -self.get_check(s).unwrap() == self.get_free_list() {
                if !self.extend_pool(self.cells.len() as TrieIndex) {
                    // unlikely
                    return None;
                }
            }
            s = -self.get_check(s).unwrap();
        }

        Some(s - first_sym as TrieIndex)
    }

    fn fit_symbols(&mut self, base: TrieIndex, symbols: &Symbols) -> bool {
        for sym in symbols.iter().copied() {
            // if (base + sym) > TRIE_INDEX_MAX which means it's overflow,
            // or cell [base + sym] is not free, the symbol is not fit.
            if base > TRIE_INDEX_MAX - sym as TrieIndex
                || !self.check_free_cell(base + sym as TrieIndex)
            {
                return false;
            }
        }
        true
    }

    fn relocate_base(&mut self, s: TrieIndex, new_base: TrieIndex) {
        let old_base = self.get_base(s).unwrap(); // was unwrap_or(TRIE_INDEX_ERROR)
        let symbols = self.output_symbols(s);

        for sym in symbols.iter().copied() {
            let old_next = old_base + sym as TrieIndex;
            let new_next = new_base + sym as TrieIndex;
            let old_next_base = self.get_base(old_next).unwrap(); // was unwrap_or

            // allocate new next node and copy BASE value
            self.alloc_cell(new_next);
            self.set_check(new_next, s);
            self.set_base(new_next, old_next_base);

            // old_next node is now moved to new_next
            // so, all cells belonging to old_next
            // must be given to new_next
            // preventing the case of TAIL pointer
            if old_next_base > 0 {
                let max_c = cmp::min(
                    TRIE_CHAR_MAX as TrieIndex,
                    self.cells.len() as TrieIndex - old_next_base,
                );
                for c in 0..=max_c {
                    if self.get_check(old_next_base + c) == Some(old_next) {
                        self.set_check(old_next_base + c, new_next);
                    }
                }
            }

            // free old_next node
            self.free_cell(old_next);
        }

        // finally, make BASE[s] point to new_base
        self.set_base(s, new_base);
    }

    fn extend_pool(&mut self, to_index: TrieIndex) -> bool {
        // Rust: minimum index is now DA_POOL_BEGIN instead of 0
        if to_index < DA_POOL_BEGIN || to_index >= TRIE_INDEX_MAX {
            return false;
        }
        if (to_index as usize) < self.cells.len() {
            return true;
        }
        // Hence get_free_list(1) < DA_POOL_BEGIN (3) < self.cells.len() <= to_index < TRIE_INDEX_MAX
        // The compiler still doesn't use this information though...

        let new_begin = self.cells.len() as TrieIndex;
        let free_tail = -self.get_base(self.get_free_list()).unwrap();

        self.cells.reserve(to_index as usize + 1 - self.cells.len());
        // XXX: The compiler currently don't unroll this loop
        // It could be faster if we generate the first and last item separately
        // but the code will be complicated
        for i in new_begin..=to_index {
            let check = if i == to_index {
                // Last index
                -self.get_free_list()
            } else {
                -(i + 1)
            };
            let base = if i == new_begin {
                // First new index
                -free_tail
            } else {
                -(i - 1)
            };
            self.cells.push(DACell { check, base })
        }
        // The compiler doesn't seems to use this information to elide bond checks below
        debug_assert_eq!(self.cells.len(), to_index as usize + 1);

        // merge the new circular list to the old
        self.set_check(free_tail, -new_begin);
        self.set_base(self.get_free_list(), -to_index);

        // update header cell
        self.cells[0].check = self.cells.len() as TrieIndex;

        true
    }

    /// Prune off a non-separate path up from the final state `s`.
    /// If `s` still has some children states, it does nothing. Otherwise,
    /// it deletes the node and all its parents which become non-separate.
    pub(crate) fn prune(&mut self, s: TrieIndex) {
        self.prune_upto(self.get_root(), s)
    }

    /// Prune off a non-separate path up from the final state `s` to the
    /// given parent `p`. The prunning stop when either the parent `p`
    /// is met, or a first non-separate node is found.
    pub(crate) fn prune_upto(&mut self, p: TrieIndex, s: TrieIndex) {
        let mut s = s;
        while p != s && !self.has_children(s) {
            let parent = self.get_check(s).unwrap();
            self.free_cell(s);
            s = parent;
        }
    }

    fn alloc_cell(&mut self, cell: TrieIndex) {
        let prev = -self.get_base(cell).unwrap();
        let next = -self.get_check(cell).unwrap();
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
        self.set_check(cell, -i);
        self.set_base(cell, -prev);
        self.set_check(prev, -cell);
        self.set_base(i, -cell);
    }

    /// Find the first separate node under a sub-trie rooted at `root` and write to `keybuff`
    ///
    /// On return, `keybuff` is appended with the key characters which walk from
    /// `root` to the separate node. This is for incrementally calculating the
    /// transition key, which is more efficient than later totally reconstructing
    /// key from the given separate node.
    pub(crate) fn first_separate(
        &self,
        root: TrieIndex,
        keybuff: &mut Vec<TrieChar>,
    ) -> Option<TrieIndex> {
        let mut root = root;
        while let Some(base) = self.get_base(root) {
            if base < 0 {
                break;
            }
            let max_c = cmp::min(
                TRIE_CHAR_MAX as TrieIndex,
                self.cells.len() as TrieIndex - base,
            );
            let c = (0..=max_c).find(|c| self.get_check(base + c) == Some(root))?;
            keybuff.push(c as TrieChar);
            root = base + c;
        }
        Some(root)
    }

    /// Find the next separate node under a sub-trie rooted at `root` starting
    /// from the current separate node `sep`.
    ///
    /// On return, `keybuff` is incrementally updated from the key which walks
    ///  to previous separate node to the one which walks to the new separate node.
    /// So, it is assumed to be initialized by at least one first_separate()
    /// call before. This incremental key calculation is more efficient than later
    /// totally reconstructing key from the given separate node.
    pub(crate) fn next_separate(
        &self,
        root: TrieIndex,
        sep: TrieIndex,
        keybuff: &mut Vec<TrieChar>,
    ) -> Option<TrieIndex> {
        let mut sep = sep;
        while sep != root {
            let parent = self.get_check(sep).unwrap();
            let base = self.get_base(parent).unwrap();
            let c = sep - base;

            keybuff.pop();

            // find next sibling of sep
            let max_c = cmp::min(
                TRIE_CHAR_MAX as TrieIndex,
                self.cells.len() as TrieIndex - base,
            );
            for c in (c + 1)..=max_c {
                if self.get_check(base + c) == Some(parent) {
                    keybuff.push(c as TrieChar);
                    return self.first_separate(base + c, keybuff);
                }
            }
            sep = parent;
        }
        None
    }

    pub(crate) fn serialized_size(&self) -> usize {
        if !self.cells.is_empty() {
            4 * self.cells.len() * 2 // `base` and `check`
        } else {
            0
        }
    }

    pub(crate) fn read<T: Read>(reader: &mut T) -> io::Result<Self> {
        // check signature
        if reader.read_i32::<BigEndian>()? != DA_SIGNATURE as i32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid signature",
            ));
        }

        // read number of cells
        let num_cells = reader.read_i32::<BigEndian>()?;
        if num_cells as isize > isize::MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cell count too large",
            ));
        }

        let mut cells = Vec::with_capacity(num_cells as usize);
        cells.push(DACell {
            base: DA_SIGNATURE as TrieIndex,
            check: num_cells,
        });

        for _ in 1..(num_cells as usize) {
            cells.push(DACell {
                base: reader.read_i32::<BigEndian>()?,
                check: reader.read_i32::<BigEndian>()?,
            });
        }

        Ok(Self { cells })
    }

    pub(crate) fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        for cell in &self.cells {
            writer.write_i32::<BigEndian>(cell.base)?;
            writer.write_i32::<BigEndian>(cell.check)?;
        }
        Ok(())
    }

    pub(crate) fn is_separate(&self, s: TrieIndex) -> bool {
        self.get_base(s).unwrap() < 0
    }

    pub(crate) fn is_walkable(&self, s: TrieIndex, c: TrieChar) -> bool {
        self.get_check(self.get_base(s).unwrap_or(TRIE_INDEX_ERROR) + c as TrieIndex)
            .unwrap_or(TRIE_INDEX_ERROR)
            == s
    }

    pub(crate) fn get_tail_index(&self, s: TrieIndex) -> TrieIndex {
        -self.get_base(s).unwrap()
    }

    pub(crate) fn set_tail_index(&mut self, s: TrieIndex, v: TrieIndex) -> Option<()> {
        self.set_base(s, -v)
    }
}

impl Default for DArray {
    fn default() -> Self {
        Self {
            cells: vec![
                DACell {
                    base: DA_SIGNATURE as TrieIndex,
                    check: 3, // length of this
                },
                DACell {
                    base: -1,
                    check: -1,
                },
                DACell {
                    base: DA_POOL_BEGIN,
                    check: 0,
                },
            ],
        }
    }
}

#[deprecated(note = "Use d.first_separate(root, keybuff).unwrap_or(TRIE_INDEX_ERROR")]
#[no_mangle]
pub(crate) extern "C" fn da_first_separate(
    d: *const DArray,
    root: TrieIndex,
    mut keybuff: NonNull<Vec<TrieChar>>,
) -> TrieIndex {
    let da = unsafe { &*d };
    let keybuff = unsafe { keybuff.as_mut() };
    da.first_separate(root, keybuff).unwrap_or(TRIE_INDEX_ERROR)
}

#[deprecated(note = "Use d.next_separate(root, sep, keybuff).unwrap_or(TRIE_INDEX_ERROR")]
#[no_mangle]
pub(crate) unsafe extern "C" fn da_next_separate(
    d: *const DArray,
    root: TrieIndex,
    sep: TrieIndex,
    mut keybuff: NonNull<Vec<TrieChar>>,
) -> TrieIndex {
    let da = unsafe { &*d };
    let keybuff = unsafe { keybuff.as_mut() };
    da.next_separate(root, sep, keybuff)
        .unwrap_or(TRIE_INDEX_ERROR)
}

use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ptr::NonNull;
use std::{cmp, io, ptr};

use ::libc;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fileutils::wrap_cfile_nonnull;
use crate::symbols::Symbols;
use crate::trie_string::{
    trie_string_append_char, trie_string_cut_last, TrieChar, TrieString, TRIE_CHAR_MAX,
};
use crate::types::*;

#[derive(Default, Clone)]
#[repr(C)]
pub(crate) struct DACell {
    base: TrieIndex,
    check: TrieIndex,
}

#[repr(C)]
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
    pub(crate) fn walk(&self, s: TrieIndex, c: TrieChar) -> Option<TrieIndex> {
        // The C code doesn't handle get_base() error here
        // either it is infallible or it abuses TRIE_INDEX_ERROR
        let next = self.get_base(s).unwrap() + c as TrieIndex;
        if self.get_check(next) == Some(s) {
            return Some(next);
        }
        None
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
        let base = self.get_base(s).unwrap_or(TRIE_INDEX_ERROR);
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

    fn find_free_base(&mut self, symbols: &Symbols) -> TrieIndex {
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
                    return TRIE_INDEX_ERROR;
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
                    return TRIE_INDEX_ERROR;
                }
            }
            s = -self.get_check(s).unwrap();
        }

        s - first_sym as TrieIndex
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

    fn extend_pool(&mut self, to_index: TrieIndex) -> bool {
        if to_index <= 0 || to_index >= TRIE_INDEX_MAX {
            return false;
        }
        if to_index < self.cells.len() as TrieIndex {
            return true;
        }

        let new_begin = self.cells.len() as TrieIndex;
        self.cells
            .resize((to_index + 1) as usize, DACell::default());

        // initialize the new free list
        for i in new_begin..to_index {
            self.set_check(i, -(i + 1));
            self.set_base(i + 1, -i);
        }

        // merge the new circular list to the old
        let free_tail = -self.get_base(self.get_free_list()).unwrap();
        self.set_check(free_tail, -new_begin);
        self.set_base(new_begin, -free_tail);
        self.set_check(to_index, -self.get_free_list());
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

    pub(crate) fn get_serialized_size(&self) -> usize {
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

#[deprecated(note = "Use DArray::default()")]
#[no_mangle]
pub(crate) extern "C" fn da_new() -> *mut DArray {
    Box::into_raw(Box::new(DArray::default()))
}

#[deprecated(note = "Use DArray::read(). Careful about file position on failure!")]
#[no_mangle]
pub(crate) extern "C" fn da_fread(file: NonNull<libc::FILE>) -> *mut DArray {
    let mut file = wrap_cfile_nonnull(file);
    let save_pos = file.seek(SeekFrom::Current(0)).unwrap();

    match DArray::read(&mut file) {
        Ok(da) => Box::into_raw(Box::new(da)),
        Err(_) => {
            // Return to save_pos if read fail
            let _ = file.seek(SeekFrom::Start(save_pos));
            return ptr::null_mut();
        }
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn da_free(mut d: NonNull<DArray>) {
    drop(Box::from_raw(d.as_mut()))
}

#[deprecated(note = "Use DArray::serialize()")]
#[no_mangle]
pub(crate) extern "C" fn da_fwrite(d: *const DArray, file: NonNull<libc::FILE>) -> i32 {
    let mut file = wrap_cfile_nonnull(file);

    let da = unsafe { &*d };

    match da.serialize(&mut file) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[deprecated(note = "Use d.get_serialized_size()")]
#[no_mangle]
pub(crate) extern "C" fn da_get_serialized_size(d: *const DArray) -> usize {
    let da = unsafe { &*d };
    da.get_serialized_size()
}

#[deprecated(note = "Use DArray::serialize()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn da_serialize(d: *const DArray, mut ptr: NonNull<NonNull<[u8]>>) {
    // FIXME: [u8] type is not actually stable ABI
    let mut cursor = Cursor::new(ptr.as_mut().as_mut());
    (*d).serialize(&mut cursor).unwrap();
    // Move ptr
    ptr.write(ptr.as_ref().byte_offset(cursor.position() as isize));
}

#[deprecated(note = "Use d.get_root()")]
#[no_mangle]
pub(crate) extern "C" fn da_get_root(d: *const DArray) -> TrieIndex {
    unsafe { &*d }.get_root()
}

#[deprecated(note = "Use d.get_base().unwrap_or(TRIE_INDEX_ERROR)")]
#[no_mangle]
pub(crate) extern "C" fn da_get_base(d: *const DArray, s: TrieIndex) -> TrieIndex {
    let da = unsafe { &*d };
    da.get_base(s).unwrap_or(TRIE_INDEX_ERROR)
}

#[deprecated(note = "Use d.get_check().unwrap_or(TRIE_INDEX_ERROR)")]
#[no_mangle]
pub(crate) extern "C" fn da_get_check(d: *const DArray, s: TrieIndex) -> TrieIndex {
    let da = unsafe { &*d };
    da.get_check(s).unwrap_or(TRIE_INDEX_ERROR)
}

#[deprecated(note = "Use d.set_base() and ignore error")]
#[no_mangle]
pub(crate) extern "C" fn da_set_base(mut d: NonNull<DArray>, s: TrieIndex, val: TrieIndex) {
    let da = unsafe { d.as_mut() };
    let _ = da.set_base(s, val);
}

#[deprecated(note = "Use d.set_check() and ignore error")]
#[no_mangle]
pub unsafe extern "C" fn da_set_check(mut d: NonNull<DArray>, s: TrieIndex, val: TrieIndex) {
    let da = unsafe { d.as_mut() };
    let _ = da.set_check(s, val);
}

#[deprecated(note = "Use Some(*s) = d.walk(s, c)")]
#[no_mangle]
pub unsafe extern "C" fn da_walk(d: *const DArray, s: *mut TrieIndex, c: TrieChar) -> Bool {
    let da = unsafe { &*d };
    if let Some(new_s) = da.walk(unsafe { *s }, c) {
        unsafe {
            *s = new_s;
        }
        return TRUE;
    }
    FALSE
}

#[no_mangle]
pub unsafe extern "C" fn da_insert_branch(
    mut d: NonNull<DArray>,
    s: TrieIndex,
    c: TrieChar,
) -> TrieIndex {
    // TODO: Port
    let da = unsafe { d.as_mut() };
    let mut next: TrieIndex = 0;
    let base = da.get_base(s).unwrap_or(TRIE_INDEX_ERROR);
    if base > TRIE_INDEX_ERROR {
        next = base + c as libc::c_int;
        if da.get_check(next) == Some(s) {
            return next;
        }
        if base > TRIE_INDEX_MAX - c as libc::c_int || !da.check_free_cell(next) {
            let mut symbols = Symbols::default();
            let mut new_base: TrieIndex = 0;
            symbols = da.output_symbols(s);
            symbols.add(c);
            new_base = da.find_free_base(&symbols);
            if 0 as libc::c_int == new_base {
                return TRIE_INDEX_ERROR;
            }
            da_relocate_base(d, s, new_base);
            next = new_base + c as libc::c_int;
        }
    } else {
        let mut new_base_0: TrieIndex = 0;
        let mut symbols_0 = Symbols::default();
        symbols_0.add(c);
        new_base_0 = da.find_free_base(&symbols_0);
        if 0 as libc::c_int == new_base_0 {
            return TRIE_INDEX_ERROR;
        }
        da.set_base(s, new_base_0);
        next = new_base_0 + c as libc::c_int;
    }
    da.alloc_cell(next);
    da.set_check(next, s);
    return next;
}

#[deprecated(note = "Use d.check_free_cell()")]
fn da_check_free_cell(mut d: NonNull<DArray>, s: TrieIndex) -> Bool {
    let da = unsafe { d.as_mut() };
    da.check_free_cell(s).into()
}

#[deprecated(note = "Use d.has_children()")]
unsafe fn da_has_children(d: *const DArray, s: TrieIndex) -> Bool {
    let da = unsafe { &*d };
    da.has_children(s).into()
}

#[deprecated(note = "Use d.output_symbols()")]
pub(crate) unsafe fn da_output_symbols(d: *const DArray, s: TrieIndex) -> Symbols {
    let da = unsafe { &*d };
    da.output_symbols(s)
}

#[deprecated(note = "Use d.find_free_base()")]
fn da_find_free_base(mut d: NonNull<DArray>, symbols: &Symbols) -> TrieIndex {
    let da = unsafe { d.as_mut() };
    da.find_free_base(symbols)
}

#[deprecated(note = "Use d.fit_symbols()")]
unsafe fn da_fit_symbols(mut d: NonNull<DArray>, base: TrieIndex, symbols: &Symbols) -> Bool {
    let da = unsafe { d.as_mut() };
    da.fit_symbols(base, symbols).into()
}

unsafe fn da_relocate_base(mut d: NonNull<DArray>, s: TrieIndex, new_base: TrieIndex) {
    // TODO: Port
    let da = unsafe { d.as_mut() };
    let mut old_base: TrieIndex = 0;
    let mut i: libc::c_int = 0;
    old_base = da.get_base(s).unwrap_or(TRIE_INDEX_ERROR);
    let symbols = da.output_symbols(s);
    i = 0 as libc::c_int;
    while i < symbols.num() as i32 {
        let mut old_next: TrieIndex = 0;
        let mut new_next: TrieIndex = 0;
        let mut old_next_base: TrieIndex = 0;
        old_next = old_base + symbols.get(i as usize).unwrap() as libc::c_int;
        new_next = new_base + symbols.get(i as usize).unwrap() as libc::c_int;
        old_next_base = da.get_base(old_next).unwrap_or(TRIE_INDEX_ERROR);
        da.alloc_cell(new_next);
        da.set_check(new_next, s);
        da.set_base(new_next, old_next_base);
        if old_next_base > 0 as libc::c_int {
            let mut c: TrieIndex = 0;
            let mut max_c: TrieIndex = 0;
            max_c = cmp::min(
                TRIE_CHAR_MAX as TrieIndex,
                da.cells.len() as TrieIndex - old_next_base,
            );
            c = 0 as libc::c_int;
            while c <= max_c {
                if da.get_check(old_next_base + c) == Some(old_next) {
                    da.set_check(old_next_base + c, new_next);
                }
                c += 1;
            }
        }
        da.free_cell(old_next);
        i += 1;
    }
    da.set_base(s, new_base);
}

#[deprecated(note = "Use da.extend_pool()")]
fn da_extend_pool(mut d: NonNull<DArray>, to_index: TrieIndex) -> Bool {
    let da = unsafe { d.as_mut() };
    da.extend_pool(to_index).into()
}

#[deprecated(note = "Use d.prune()")]
#[no_mangle]
pub(crate) extern "C" fn da_prune(mut d: NonNull<DArray>, s: TrieIndex) {
    let da = unsafe { d.as_mut() };
    da.prune(s)
}

#[deprecated(note = "Use d.prune_upto()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn da_prune_upto(mut d: NonNull<DArray>, p: TrieIndex, s: TrieIndex) {
    let da = unsafe { d.as_mut() };
    da.prune_upto(p, s)
}

#[deprecated(note = "Use d.alloc_cell()")]
fn da_alloc_cell(mut d: NonNull<DArray>, cell: TrieIndex) {
    let da = unsafe { d.as_mut() };
    da.alloc_cell(cell)
}

#[deprecated(note = "Use d.free_cell()")]
fn da_free_cell(mut d: NonNull<DArray>, cell: TrieIndex) {
    let da = unsafe { d.as_mut() };
    da.free_cell(cell)
}

#[no_mangle]
pub unsafe extern "C" fn da_first_separate(
    mut d: NonNull<DArray>,
    mut root: TrieIndex,
    keybuff: *mut TrieString,
) -> TrieIndex {
    let da = unsafe { d.as_mut() };
    // TODO: Port
    let mut base: TrieIndex = 0;
    let mut c: TrieIndex = 0;
    let mut max_c: TrieIndex = 0;
    loop {
        base = da.get_base(root).unwrap_or(TRIE_INDEX_ERROR);
        if !(base >= 0 as libc::c_int) {
            break;
        }
        max_c = cmp::min(
            TRIE_CHAR_MAX as TrieIndex,
            da.cells.len() as TrieIndex - base,
        );
        c = 0 as libc::c_int;
        while c <= max_c {
            if da.get_check(base + c) == Some(root) {
                break;
            }
            c += 1;
        }
        if c > max_c {
            return TRIE_INDEX_ERROR;
        }
        trie_string_append_char(keybuff, c as TrieChar);
        root = base + c;
    }
    return root;
}

#[no_mangle]
pub unsafe extern "C" fn da_next_separate(
    mut d: NonNull<DArray>,
    root: TrieIndex,
    mut sep: TrieIndex,
    keybuff: *mut TrieString,
) -> TrieIndex {
    let da = unsafe { d.as_mut() };
    // TODO: Port
    let mut parent: TrieIndex = 0;
    let mut base: TrieIndex = 0;
    let mut c: TrieIndex = 0;
    let mut max_c: TrieIndex = 0;
    while sep != root {
        parent = da.get_check(sep).unwrap_or(TRIE_INDEX_ERROR);
        base = da.get_base(parent).unwrap_or(TRIE_INDEX_ERROR);
        c = sep - base;
        trie_string_cut_last(keybuff);
        max_c = cmp::min(
            TRIE_CHAR_MAX as TrieIndex,
            da.cells.len() as TrieIndex - base,
        );
        loop {
            c += 1;
            if !(c <= max_c) {
                break;
            }
            if da.get_check(base + c) == Some(parent) {
                trie_string_append_char(keybuff, c as TrieChar);
                return da_first_separate(d, base + c, keybuff);
            }
        }
        sep = parent;
    }
    return TRIE_INDEX_ERROR;
}

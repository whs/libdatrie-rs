use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ptr::NonNull;
use std::{cmp, io, ptr, slice};

use ::libc;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fileutils::wrap_cfile_nonnull;
use crate::symbols::Symbols;
use crate::trie_string::{
    trie_string_append_char, trie_string_cut_last, TrieChar, TrieString, TRIE_CHAR_MAX,
};
use crate::types::*;

extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}

#[repr(C)]
pub(crate) struct DACell {
    base: TrieIndex,
    check: TrieIndex,
}

#[repr(C)]
pub(crate) struct DArray {
    num_cells: TrieIndex,
    // TODO: This should be Vec
    cells: *mut DACell,
}

const DA_SIGNATURE: u32 = 0xdafcdafc;

// DA Header:
// - Cell 0: SIGNATURE, number of cells
// - Cell 1: free circular-list pointers
// - Cell 2: root node
// - Cell 3: DA pool begin
const DA_POOL_BEGIN: TrieIndex = 3;

impl DArray {
    fn slice(&self) -> &[DACell] {
        unsafe { slice::from_raw_parts(self.cells, self.num_cells as usize) }
    }

    fn slice_mut(&self) -> &mut [DACell] {
        unsafe { slice::from_raw_parts_mut(self.cells, self.num_cells as usize) }
    }

    pub(crate) fn get_free_list(&self) -> TrieIndex {
        1
    }

    pub(crate) fn get_root(&self) -> TrieIndex {
        2
    }

    /// Get BASE cell value for the given state.
    pub(crate) fn get_base(&self, s: TrieIndex) -> Option<TrieIndex> {
        self.slice().get(s as usize).map(|v| v.base)
    }

    /// Set BASE cell for the given state to the given value.
    pub(crate) fn set_base(&mut self, s: TrieIndex, val: TrieIndex) -> Option<()> {
        match self.slice_mut().get_mut(s as usize) {
            Some(cell) => {
                cell.base = val;
                Some(())
            }
            None => None,
        }
    }

    /// Get CHECK cell value for the given state.
    pub(crate) fn get_check(&self, s: TrieIndex) -> Option<TrieIndex> {
        self.slice().get(s as usize).map(|v| v.check)
    }

    /// Set CHECK cell for the given state to the given value.
    pub(crate) fn set_check(&mut self, s: TrieIndex, val: TrieIndex) -> Option<()> {
        match self.slice_mut().get_mut(s as usize) {
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
        unsafe {
            da_extend_pool(self, s).into() && self.get_check(s).unwrap_or(TRIE_INDEX_ERROR) < 0
        }
    }

    fn has_children(&self, s: TrieIndex) -> bool {
        let Some(base) = self.get_base(s) else {
            return false;
        };
        if base < 0 {
            return false;
        }
        let max_c = cmp::min(TRIE_CHAR_MAX as TrieIndex, self.num_cells - base);
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
        let max_c = cmp::min(TrieChar::MAX as TrieIndex, self.num_cells - base);
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
                if unsafe { !da_extend_pool(self, s) } {
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
                if unsafe { !da_extend_pool(self, self.num_cells) } {
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
        if self.num_cells > 0 {
            4 * self.num_cells as usize * 2 // `base` and `check`
        } else {
            0
        }
    }

    pub(crate) fn read<T: Read>(reader: &mut T) -> io::Result<Self> {
        if reader.read_i32::<BigEndian>()? != DA_SIGNATURE as i32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid signature",
            ));
        }

        let num_cells = reader.read_i32::<BigEndian>()?;
        if num_cells as isize > isize::MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cell count too large",
            ));
        }

        let cells: *mut DACell =
            unsafe { malloc((num_cells as usize * size_of::<DACell>()) as libc::c_ulong).cast() };
        let cells_slice = unsafe { slice::from_raw_parts_mut(cells, num_cells as usize) };

        cells_slice[0].base = DA_SIGNATURE as TrieIndex;
        cells_slice[0].check = num_cells;

        for i in 1..(num_cells as usize) {
            cells_slice[i].base = reader.read_i32::<BigEndian>()?;
            cells_slice[i].check = reader.read_i32::<BigEndian>()?;
        }

        Ok(Self { num_cells, cells })
    }

    pub(crate) fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        for cell in self.slice() {
            writer.write_i32::<BigEndian>(cell.base)?;
            writer.write_i32::<BigEndian>(cell.check)?;
        }
        Ok(())
    }
}

impl Default for DArray {
    fn default() -> Self {
        let len = DA_POOL_BEGIN;
        let cells_ptr: *mut DACell =
            unsafe { malloc((len as usize * size_of::<DACell>()) as libc::c_ulong).cast() };
        let cells = unsafe { slice::from_raw_parts_mut(cells_ptr, len as usize) };

        cells[0].base = DA_SIGNATURE as TrieIndex;
        cells[0].check = len;

        cells[1].base = -1;
        cells[1].check = -1;

        cells[2].base = DA_POOL_BEGIN;
        cells[2].check = 0;

        Self {
            num_cells: DA_POOL_BEGIN,
            cells: cells_ptr,
        }
    }
}

impl Drop for DArray {
    fn drop(&mut self) {
        unsafe {
            free(self.cells.cast());
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
pub(crate) extern "C" fn da_fread(mut file: NonNull<libc::FILE>) -> *mut DArray {
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
pub(crate) extern "C" fn da_fwrite(d: *const DArray, mut file: NonNull<libc::FILE>) -> i32 {
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
            symbols = da_output_symbols(d.as_ref(), s);
            symbols.add(c);
            new_base = da_find_free_base(da.into(), &symbols);
            if 0 as libc::c_int == new_base {
                return TRIE_INDEX_ERROR;
            }
            da_relocate_base(da, s, new_base);
            next = new_base + c as libc::c_int;
        }
    } else {
        let mut new_base_0: TrieIndex = 0;
        let mut symbols_0 = Symbols::default();
        symbols_0.add(c);
        new_base_0 = da_find_free_base(da.into(), &symbols_0);
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

unsafe fn da_relocate_base(mut d: *mut DArray, mut s: TrieIndex, mut new_base: TrieIndex) {
    // TODO: Port
    let mut old_base: TrieIndex = 0;
    let mut i: libc::c_int = 0;
    old_base = da_get_base(d, s);
    let symbols = da_output_symbols(d, s);
    i = 0 as libc::c_int;
    while i < symbols.num() as i32 {
        let mut old_next: TrieIndex = 0;
        let mut new_next: TrieIndex = 0;
        let mut old_next_base: TrieIndex = 0;
        old_next = old_base + symbols.get(i as usize).unwrap() as libc::c_int;
        new_next = new_base + symbols.get(i as usize).unwrap() as libc::c_int;
        old_next_base = da_get_base(d, old_next);
        da_alloc_cell(NonNull::new_unchecked(d), new_next);
        da_set_check(NonNull::new_unchecked(d), new_next, s);
        da_set_base(NonNull::new_unchecked(d), new_next, old_next_base);
        if old_next_base > 0 as libc::c_int {
            let mut c: TrieIndex = 0;
            let mut max_c: TrieIndex = 0;
            max_c = if (255 as libc::c_int) < (*d).num_cells - old_next_base {
                255 as libc::c_int
            } else {
                (*d).num_cells - old_next_base
            };
            c = 0 as libc::c_int;
            while c <= max_c {
                if da_get_check(d, old_next_base + c) == old_next {
                    da_set_check(NonNull::new_unchecked(d), old_next_base + c, new_next);
                }
                c += 1;
                c;
            }
        }
        da_free_cell(NonNull::new_unchecked(d), old_next);
        i += 1;
        i;
    }
    da_set_base(NonNull::new_unchecked(d), s, new_base);
}

unsafe fn da_extend_pool(mut d: *mut DArray, mut to_index: TrieIndex) -> Bool {
    // TODO: Port
    let mut new_block: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_begin: TrieIndex = 0;
    let mut i: TrieIndex = 0;
    let mut free_tail: TrieIndex = 0;
    if to_index <= 0 as libc::c_int || 0x7fffffff as libc::c_int <= to_index {
        return FALSE as Bool;
    }
    if to_index < (*d).num_cells {
        return TRUE as Bool;
    }
    new_block = realloc(
        (*d).cells as *mut libc::c_void,
        ((to_index + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<DACell>() as libc::c_ulong),
    );
    if new_block.is_null() {
        return FALSE as Bool;
    }
    (*d).cells = new_block as *mut DACell;
    new_begin = (*d).num_cells;
    (*d).num_cells = to_index + 1 as libc::c_int;
    i = new_begin;
    while i < to_index {
        da_set_check(NonNull::new_unchecked(d), i, -(i + 1 as libc::c_int));
        da_set_base(NonNull::new_unchecked(d), i + 1 as libc::c_int, -i);
        i += 1;
        i;
    }
    free_tail = -da_get_base(d, 1 as libc::c_int);
    da_set_check(NonNull::new_unchecked(d), free_tail, -new_begin);
    da_set_base(NonNull::new_unchecked(d), new_begin, -free_tail);
    da_set_check(NonNull::new_unchecked(d), to_index, -(1 as libc::c_int));
    da_set_base(NonNull::new_unchecked(d), 1 as libc::c_int, -to_index);
    (*((*d).cells).offset(0 as libc::c_int as isize)).check = (*d).num_cells;
    return TRUE as Bool;
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
    mut d: *mut DArray,
    mut root: TrieIndex,
    mut keybuff: *mut TrieString,
) -> TrieIndex {
    // TODO: Port
    let mut base: TrieIndex = 0;
    let mut c: TrieIndex = 0;
    let mut max_c: TrieIndex = 0;
    loop {
        base = da_get_base(d, root);
        if !(base >= 0 as libc::c_int) {
            break;
        }
        max_c = if (255 as libc::c_int) < (*d).num_cells - base {
            255 as libc::c_int
        } else {
            (*d).num_cells - base
        };
        c = 0 as libc::c_int;
        while c <= max_c {
            if da_get_check(d, base + c) == root {
                break;
            }
            c += 1;
            c;
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
    mut d: *mut DArray,
    mut root: TrieIndex,
    mut sep: TrieIndex,
    mut keybuff: *mut TrieString,
) -> TrieIndex {
    // TODO: Port
    let mut parent: TrieIndex = 0;
    let mut base: TrieIndex = 0;
    let mut c: TrieIndex = 0;
    let mut max_c: TrieIndex = 0;
    while sep != root {
        parent = da_get_check(d, sep);
        base = da_get_base(d, parent);
        c = sep - base;
        trie_string_cut_last(keybuff);
        max_c = if (255 as libc::c_int) < (*d).num_cells - base {
            255 as libc::c_int
        } else {
            (*d).num_cells - base
        };
        loop {
            c += 1;
            if !(c <= max_c) {
                break;
            }
            if da_get_check(d, base + c) == parent {
                trie_string_append_char(keybuff, c as TrieChar);
                return da_first_separate(d, base + c, keybuff);
            }
        }
        sep = parent;
    }
    return TRIE_INDEX_ERROR;
}

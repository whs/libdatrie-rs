use std::{cmp, ptr, slice};
use std::ptr::NonNull;
use ::libc;

use crate::fileutils::{file_read_int32, file_write_int32, serialize_int32_be_incr};
use crate::symbols::Symbols;
use crate::trie::TRIE_INDEX_ERROR;
use crate::trie_string::{trie_string_append_char, trie_string_cut_last, TrieChar, TrieString};
use crate::types::*;

extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
}
pub type size_t = libc::c_ulong;
pub type FILE = libc::FILE;
pub type uint8 = u8;
pub type uint32 = u32;
pub type int32 = i32;

pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;

#[repr(C)]
pub(crate) struct DACell {
    base: TrieIndex,
    check: TrieIndex,
}

#[repr(C)]
pub(crate) struct DArray {
    num_cells: TrieIndex,
    // This should be Vec
    cells: *mut DACell,
}

pub(crate) const DA_SIGNATURE: u32 = 0xdafcdafc;

// DA Header:
// - Cell 0: SIGNATURE, number of cells
// - Cell 1: free circular-list pointers
// - Cell 2: root node
// - Cell 3: DA pool begin
pub(crate) const DA_POOL_BEGIN: TrieIndex = 3;

impl Default for DArray {
    fn default() -> Self {
        let mut d = Self {
            num_cells: DA_POOL_BEGIN,
            cells: ptr::null_mut(),
        };

        d.cells = unsafe { malloc((d.num_cells as usize * size_of::<DACell>()) as libc::c_ulong).cast() };
        let cells = unsafe { slice::from_raw_parts_mut(d.cells, d.num_cells as usize) };

        cells[0].base = DA_SIGNATURE as TrieIndex;
        cells[0].check = d.num_cells;

        cells[1].base = -1;
        cells[1].check = -1;

        cells[2].base = DA_POOL_BEGIN;
        cells[2].check = 0;

        d
    }
}

impl Drop for DArray {
    fn drop(&mut self) {
        unsafe {
            free(self.cells.cast());
        }
    }
}

#[no_mangle]
pub extern "C" fn da_new() -> *mut DArray {
    Box::into_raw(Box::new(DArray::default()))
}

#[no_mangle]
pub unsafe extern "C" fn da_fread(mut file: *mut FILE) -> *mut DArray {
    let mut current_block: u64;
    let mut save_pos: libc::c_long = 0;
    let mut d: *mut DArray = NULL as *mut DArray;
    let mut n: TrieIndex = 0;
    save_pos = ftell(file);
    if !(file_read_int32(file, &mut n) as u64 == 0 || DA_SIGNATURE != n as uint32) {
        d = malloc(size_of::<DArray>() as libc::c_ulong) as *mut DArray;
        if !d.is_null() {
            if !(file_read_int32(file, &mut (*d).num_cells) as u64 == 0) {
                if !((*d).num_cells as libc::c_ulong
                    > SIZE_MAX.wrapping_div(size_of::<DACell>() as libc::c_ulong))
                {
                    (*d).cells = malloc(((*d).num_cells as libc::c_ulong).wrapping_mul(size_of::<
                        DACell,
                    >(
                    )
                        as libc::c_ulong)) as *mut DACell;
                    if !((*d).cells).is_null() {
                        (*((*d).cells).offset(0 as libc::c_int as isize)).base =
                            DA_SIGNATURE as TrieIndex;
                        (*((*d).cells).offset(0 as libc::c_int as isize)).check = (*d).num_cells;
                        n = 1 as libc::c_int;
                        loop {
                            if !(n < (*d).num_cells) {
                                current_block = 11050875288958768710;
                                break;
                            }
                            if file_read_int32(file, &mut (*((*d).cells).offset(n as isize)).base)
                                as u64
                                == 0
                                || file_read_int32(
                                    file,
                                    &mut (*((*d).cells).offset(n as isize)).check,
                                ) as u64
                                    == 0
                            {
                                current_block = 3625861430878857304;
                                break;
                            }
                            n += 1;
                            n;
                        }
                        match current_block {
                            11050875288958768710 => return d,
                            _ => {
                                free((*d).cells as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
            free(d as *mut libc::c_void);
        }
    }
    fseek(file, save_pos, SEEK_SET);
    return NULL as *mut DArray;
}

#[no_mangle]
pub(crate) unsafe extern "C" fn da_free(mut d: NonNull<DArray>) {
    drop(Box::from_raw(d.as_mut()))
}

#[no_mangle]
pub unsafe extern "C" fn da_fwrite(mut d: *const DArray, mut file: *mut FILE) -> libc::c_int {
    let mut i: TrieIndex = 0;
    i = 0 as libc::c_int;
    while i < (*d).num_cells {
        if file_write_int32(file, (*((*d).cells).offset(i as isize)).base) as u64 == 0
            || file_write_int32(file, (*((*d).cells).offset(i as isize)).check) as u64 == 0
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub extern "C" fn da_get_serialized_size(d: *const DArray) -> usize {
    // TODO: Move into the struct
    let da = unsafe {&*d};
    if da.num_cells > 0 {
        4 * da.num_cells as usize * 2 // `base` and `check`
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn da_serialize(mut d: *const DArray, mut ptr: *mut *mut uint8) {
    let mut i: TrieIndex = 0;
    i = 0 as libc::c_int;
    while i < (*d).num_cells {
        serialize_int32_be_incr(ptr, (*((*d).cells).offset(i as isize)).base);
        serialize_int32_be_incr(ptr, (*((*d).cells).offset(i as isize)).check);
        i += 1;
        i;
    }
}

#[no_mangle]
pub extern "C" fn da_get_root(d: *const DArray) -> TrieIndex {
    // TODO: Move into the struct
    2
}

#[no_mangle]
pub unsafe extern "C" fn da_get_base(mut d: *const DArray, mut s: TrieIndex) -> TrieIndex {
    return if s < (*d).num_cells {
        (*((*d).cells).offset(s as isize)).base
    } else {
        TRIE_INDEX_ERROR
    };
}

#[no_mangle]
pub unsafe extern "C" fn da_get_check(mut d: *const DArray, mut s: TrieIndex) -> TrieIndex {
    return if s < (*d).num_cells {
        (*((*d).cells).offset(s as isize)).check
    } else {
        TRIE_INDEX_ERROR
    };
}

#[no_mangle]
pub unsafe extern "C" fn da_set_base(mut d: *mut DArray, mut s: TrieIndex, mut val: TrieIndex) {
    if s < (*d).num_cells {
        (*((*d).cells).offset(s as isize)).base = val;
    }
}

#[no_mangle]
pub unsafe extern "C" fn da_set_check(mut d: *mut DArray, mut s: TrieIndex, mut val: TrieIndex) {
    if s < (*d).num_cells {
        (*((*d).cells).offset(s as isize)).check = val;
    }
}

#[no_mangle]
pub unsafe extern "C" fn da_walk(
    mut d: *const DArray,
    mut s: *mut TrieIndex,
    mut c: TrieChar,
) -> Bool {
    let mut next: TrieIndex = 0;
    next = da_get_base(d, *s) + c as libc::c_int;
    if da_get_check(d, next) == *s {
        *s = next;
        return TRUE as Bool;
    }
    return FALSE as Bool;
}

#[no_mangle]
pub unsafe extern "C" fn da_insert_branch(
    mut d: *mut DArray,
    mut s: TrieIndex,
    mut c: TrieChar,
) -> TrieIndex {
    let mut base: TrieIndex = 0;
    let mut next: TrieIndex = 0;
    base = da_get_base(d, s);
    if base > 0 as libc::c_int {
        next = base + c as libc::c_int;
        if da_get_check(d, next) == s {
            return next;
        }
        if base > TRIE_INDEX_MAX - c as libc::c_int || da_check_free_cell(d, next) as u64 == 0 {
            let mut symbols = Symbols::default();
            let mut new_base: TrieIndex = 0;
            symbols = da_output_symbols(d, s);
            symbols.add(c);
            new_base = da_find_free_base(d, &symbols);
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
        new_base_0 = da_find_free_base(d, &symbols_0);
        if 0 as libc::c_int == new_base_0 {
            return TRIE_INDEX_ERROR;
        }
        da_set_base(d, s, new_base_0);
        next = new_base_0 + c as libc::c_int;
    }
    da_alloc_cell(d, next);
    da_set_check(d, next, s);
    return next;
}

unsafe fn da_check_free_cell(mut d: *mut DArray, mut s: TrieIndex) -> Bool {
    return (da_extend_pool(d, s) as libc::c_uint != 0 && da_get_check(d, s) < 0 as libc::c_int)
        as libc::c_int as Bool;
}

unsafe fn da_has_children(mut d: *const DArray, mut s: TrieIndex) -> Bool {
    let mut base: TrieIndex = 0;
    let mut c: TrieIndex = 0;
    let mut max_c: TrieIndex = 0;
    base = da_get_base(d, s);
    if TRIE_INDEX_ERROR == base || base < 0 as libc::c_int {
        return FALSE as Bool;
    }
    max_c = if (255 as libc::c_int) < (*d).num_cells - base {
        255 as libc::c_int
    } else {
        (*d).num_cells - base
    };
    c = 0 as libc::c_int;
    while c <= max_c {
        if da_get_check(d, base + c) == s {
            return TRUE as Bool;
        }
        c += 1;
        c;
    }
    return FALSE as Bool;
}

pub(crate) unsafe fn da_output_symbols(mut d: *const DArray, mut s: TrieIndex) -> Symbols {
    let mut syms = Symbols::default();
    let base = da_get_base(d, s);
    let max_c = cmp::min(TrieChar::MAX as TrieIndex, (*d).num_cells - base);
    let mut c = 0;
    // TODO: Change while to for
    while c <= max_c {
        if da_get_check(d, base + c) == s {
            syms.add_fast(c as TrieChar);
        }
        c += 1;
    }
    syms
}

unsafe fn da_find_free_base(mut d: *mut DArray, symbols: &Symbols) -> TrieIndex {
    let mut first_sym: TrieChar = 0;
    let mut s: TrieIndex = 0;
    first_sym = symbols.get(0).unwrap();
    s = -da_get_check(d, 1 as libc::c_int);
    while s != 1 as libc::c_int && s < first_sym as TrieIndex + DA_POOL_BEGIN {
        s = -da_get_check(d, s);
    }
    if s == 1 as libc::c_int {
        s = first_sym as libc::c_int + DA_POOL_BEGIN;
        loop {
            if da_extend_pool(d, s) as u64 == 0 {
                return TRIE_INDEX_ERROR;
            }
            if da_get_check(d, s) < 0 as libc::c_int {
                break;
            }
            s += 1;
            s;
        }
    }
    while da_fit_symbols(d, s - first_sym as libc::c_int, symbols) as u64 == 0 {
        if -da_get_check(d, s) == 1 as libc::c_int {
            if da_extend_pool(d, (*d).num_cells) as u64 == 0 {
                return TRIE_INDEX_ERROR;
            }
        }
        s = -da_get_check(d, s);
    }
    return s - first_sym as libc::c_int;
}

unsafe fn da_fit_symbols(
    mut d: *mut DArray,
    mut base: TrieIndex,
    symbols: &Symbols,
) -> Bool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < symbols.num() as i32 {
        let mut sym: TrieChar = symbols.get(i as usize).unwrap();
        if base > TRIE_INDEX_MAX - sym as libc::c_int
            || da_check_free_cell(d, base + sym as libc::c_int) as u64 == 0
        {
            return FALSE as Bool;
        }
        i += 1;
        i;
    }
    return TRUE as Bool;
}

unsafe fn da_relocate_base(
    mut d: *mut DArray,
    mut s: TrieIndex,
    mut new_base: TrieIndex,
) {
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
        da_alloc_cell(d, new_next);
        da_set_check(d, new_next, s);
        da_set_base(d, new_next, old_next_base);
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
                    da_set_check(d, old_next_base + c, new_next);
                }
                c += 1;
                c;
            }
        }
        da_free_cell(d, old_next);
        i += 1;
        i;
    }
    da_set_base(d, s, new_base);
}

unsafe fn da_extend_pool(mut d: *mut DArray, mut to_index: TrieIndex) -> Bool {
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
        da_set_check(d, i, -(i + 1 as libc::c_int));
        da_set_base(d, i + 1 as libc::c_int, -i);
        i += 1;
        i;
    }
    free_tail = -da_get_base(d, 1 as libc::c_int);
    da_set_check(d, free_tail, -new_begin);
    da_set_base(d, new_begin, -free_tail);
    da_set_check(d, to_index, -(1 as libc::c_int));
    da_set_base(d, 1 as libc::c_int, -to_index);
    (*((*d).cells).offset(0 as libc::c_int as isize)).check = (*d).num_cells;
    return TRUE as Bool;
}

#[no_mangle]
pub unsafe extern "C" fn da_prune(mut d: *mut DArray, mut s: TrieIndex) {
    da_prune_upto(d, da_get_root(d), s);
}

#[no_mangle]
pub unsafe extern "C" fn da_prune_upto(mut d: *mut DArray, mut p: TrieIndex, mut s: TrieIndex) {
    while p != s && da_has_children(d, s) as u64 == 0 {
        let mut parent: TrieIndex = 0;
        parent = da_get_check(d, s);
        da_free_cell(d, s);
        s = parent;
    }
}

unsafe fn da_alloc_cell(mut d: *mut DArray, mut cell: TrieIndex) {
    let mut prev: TrieIndex = 0;
    let mut next: TrieIndex = 0;
    prev = -da_get_base(d, cell);
    next = -da_get_check(d, cell);
    da_set_check(d, prev, -next);
    da_set_base(d, next, -prev);
}

unsafe fn da_free_cell(mut d: *mut DArray, mut cell: TrieIndex) {
    let mut i: TrieIndex = 0;
    let mut prev: TrieIndex = 0;
    i = -da_get_check(d, 1 as libc::c_int);
    while i != 1 as libc::c_int && i < cell {
        i = -da_get_check(d, i);
    }
    prev = -da_get_base(d, i);
    da_set_check(d, cell, -i);
    da_set_base(d, cell, -prev);
    da_set_check(d, prev, -cell);
    da_set_base(d, i, -cell);
}

#[no_mangle]
pub unsafe extern "C" fn da_first_separate(
    mut d: *mut DArray,
    mut root: TrieIndex,
    mut keybuff: *mut TrieString,
) -> TrieIndex {
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

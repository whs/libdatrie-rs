use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _TrieString;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn trie_string_append_char(ts: *mut TrieString, tc: TrieChar) -> Bool;
    fn trie_string_cut_last(ts: *mut TrieString) -> Bool;
    fn serialize_int32_be_incr(buff: *mut *mut uint8, val: int32);
    fn file_read_int32(file: *mut FILE, o_val: *mut int32) -> Bool;
    fn file_write_int32(file: *mut FILE, val: int32) -> Bool;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type TrieChar = libc::c_uchar;
pub type TrieIndex = int32;
pub type TrieString = _TrieString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Symbols {
    pub num_symbols: libc::c_short,
    pub symbols: [TrieChar; 256],
}
pub type Symbols = _Symbols;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DArray {
    pub num_cells: TrieIndex,
    pub cells: *mut DACell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DACell {
    pub base: TrieIndex,
    pub check: TrieIndex,
}
pub type DArray = _DArray;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong)
        >> 56 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong) >> 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong) >> 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong) >> 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong) << 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56 as libc::c_int)
        as __uint64_t;
}
#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const FALSE: libc::c_int = DA_FALSE as libc::c_int;
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
pub const TRIE_INDEX_MAX: libc::c_int = 0x7fffffff as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const TRIE_INDEX_ERROR: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn symbols_new() -> *mut Symbols {
    let mut syms: *mut Symbols = 0 as *mut Symbols;
    syms = malloc(::core::mem::size_of::<Symbols>() as libc::c_ulong) as *mut Symbols;
    if syms.is_null() {
        return NULL as *mut Symbols;
    }
    (*syms).num_symbols = 0 as libc::c_int as libc::c_short;
    return syms;
}
#[no_mangle]
pub unsafe extern "C" fn symbols_free(mut syms: *mut Symbols) {
    free(syms as *mut libc::c_void);
}
unsafe extern "C" fn symbols_add(mut syms: *mut Symbols, mut c: TrieChar) {
    let mut lower: libc::c_short = 0;
    let mut upper: libc::c_short = 0;
    lower = 0 as libc::c_int as libc::c_short;
    upper = (*syms).num_symbols;
    while (lower as libc::c_int) < upper as libc::c_int {
        let mut middle: libc::c_short = 0;
        middle =
            ((lower as libc::c_int + upper as libc::c_int) / 2 as libc::c_int) as libc::c_short;
        if c as libc::c_int > (*syms).symbols[middle as usize] as libc::c_int {
            lower = (middle as libc::c_int + 1 as libc::c_int) as libc::c_short;
        } else if (c as libc::c_int) < (*syms).symbols[middle as usize] as libc::c_int {
            upper = middle;
        } else {
            return;
        }
    }
    if (lower as libc::c_int) < (*syms).num_symbols as libc::c_int {
        memmove(
            ((*syms).symbols)
                .as_mut_ptr()
                .offset(lower as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as *mut libc::c_void,
            ((*syms).symbols)
                .as_mut_ptr()
                .offset(lower as libc::c_int as isize) as *const libc::c_void,
            ((*syms).num_symbols as libc::c_int - lower as libc::c_int) as libc::c_ulong,
        );
    }
    (*syms).symbols[lower as usize] = c;
    (*syms).num_symbols += 1;
    (*syms).num_symbols;
}
#[no_mangle]
pub unsafe extern "C" fn symbols_num(mut syms: *const Symbols) -> libc::c_int {
    return (*syms).num_symbols as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn symbols_get(mut syms: *const Symbols, mut index: libc::c_int) -> TrieChar {
    return (*syms).symbols[index as usize];
}
pub const DA_SIGNATURE: libc::c_uint = 0xdafcdafc as libc::c_uint;
pub const DA_POOL_BEGIN: libc::c_int = 3 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn da_new() -> *mut DArray {
    let mut d: *mut DArray = 0 as *mut DArray;
    d = malloc(::core::mem::size_of::<DArray>() as libc::c_ulong) as *mut DArray;
    if d.is_null() {
        return NULL as *mut DArray;
    }
    (*d).num_cells = DA_POOL_BEGIN;
    (*d).cells = malloc(
        ((*d).num_cells as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<DACell>() as libc::c_ulong),
    ) as *mut DACell;
    if ((*d).cells).is_null() {
        free(d as *mut libc::c_void);
        return NULL as *mut DArray;
    } else {
        (*((*d).cells).offset(0 as libc::c_int as isize)).base = DA_SIGNATURE as TrieIndex;
        (*((*d).cells).offset(0 as libc::c_int as isize)).check = (*d).num_cells;
        (*((*d).cells).offset(1 as libc::c_int as isize)).base = -(1 as libc::c_int);
        (*((*d).cells).offset(1 as libc::c_int as isize)).check = -(1 as libc::c_int);
        (*((*d).cells).offset(2 as libc::c_int as isize)).base = DA_POOL_BEGIN;
        (*((*d).cells).offset(2 as libc::c_int as isize)).check = 0 as libc::c_int;
        return d;
    };
}
#[no_mangle]
pub unsafe extern "C" fn da_fread(mut file: *mut FILE) -> *mut DArray {
    let mut current_block: u64;
    let mut save_pos: libc::c_long = 0;
    let mut d: *mut DArray = NULL as *mut DArray;
    let mut n: TrieIndex = 0;
    save_pos = ftell(file);
    if !(file_read_int32(file, &mut n) as u64 == 0 || DA_SIGNATURE != n as uint32) {
        d = malloc(::core::mem::size_of::<DArray>() as libc::c_ulong) as *mut DArray;
        if !d.is_null() {
            if !(file_read_int32(file, &mut (*d).num_cells) as u64 == 0) {
                if !((*d).num_cells as libc::c_ulong
                    > SIZE_MAX.wrapping_div(::core::mem::size_of::<DACell>() as libc::c_ulong))
                {
                    (*d).cells = malloc(
                        ((*d).num_cells as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<DACell>() as libc::c_ulong),
                    ) as *mut DACell;
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
pub unsafe extern "C" fn da_free(mut d: *mut DArray) {
    free((*d).cells as *mut libc::c_void);
    free(d as *mut libc::c_void);
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
pub unsafe extern "C" fn da_get_serialized_size(mut d: *const DArray) -> size_t {
    if (*d).num_cells > 0 as libc::c_int {
        return (4 as libc::c_int * (*d).num_cells * 2 as libc::c_int) as size_t;
    } else {
        return 0 as libc::c_int as size_t;
    };
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
pub unsafe extern "C" fn da_get_root(mut d: *const DArray) -> TrieIndex {
    return 2 as libc::c_int;
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
            let mut symbols: *mut Symbols = 0 as *mut Symbols;
            let mut new_base: TrieIndex = 0;
            symbols = da_output_symbols(d, s);
            symbols_add(symbols, c);
            new_base = da_find_free_base(d, symbols);
            symbols_free(symbols);
            if 0 as libc::c_int == new_base {
                return TRIE_INDEX_ERROR;
            }
            da_relocate_base(d, s, new_base);
            next = new_base + c as libc::c_int;
        }
    } else {
        let mut symbols_0: *mut Symbols = 0 as *mut Symbols;
        let mut new_base_0: TrieIndex = 0;
        symbols_0 = symbols_new();
        symbols_add(symbols_0, c);
        new_base_0 = da_find_free_base(d, symbols_0);
        symbols_free(symbols_0);
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
unsafe extern "C" fn da_check_free_cell(mut d: *mut DArray, mut s: TrieIndex) -> Bool {
    return (da_extend_pool(d, s) as libc::c_uint != 0 && da_get_check(d, s) < 0 as libc::c_int)
        as libc::c_int as Bool;
}
unsafe extern "C" fn da_has_children(mut d: *const DArray, mut s: TrieIndex) -> Bool {
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
#[no_mangle]
pub unsafe extern "C" fn da_output_symbols(mut d: *const DArray, mut s: TrieIndex) -> *mut Symbols {
    let mut syms: *mut Symbols = 0 as *mut Symbols;
    let mut base: TrieIndex = 0;
    let mut c: TrieIndex = 0;
    let mut max_c: TrieIndex = 0;
    syms = symbols_new();
    base = da_get_base(d, s);
    max_c = if (255 as libc::c_int) < (*d).num_cells - base {
        255 as libc::c_int
    } else {
        (*d).num_cells - base
    };
    c = 0 as libc::c_int;
    while c <= max_c {
        if da_get_check(d, base + c) == s {
            let fresh0 = (*syms).num_symbols;
            (*syms).num_symbols = (*syms).num_symbols + 1;
            (*syms).symbols[fresh0 as usize] = c as TrieChar;
        }
        c += 1;
        c;
    }
    return syms;
}
unsafe extern "C" fn da_find_free_base(
    mut d: *mut DArray,
    mut symbols: *const Symbols,
) -> TrieIndex {
    let mut first_sym: TrieChar = 0;
    let mut s: TrieIndex = 0;
    first_sym = symbols_get(symbols, 0 as libc::c_int);
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
unsafe extern "C" fn da_fit_symbols(
    mut d: *mut DArray,
    mut base: TrieIndex,
    mut symbols: *const Symbols,
) -> Bool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < symbols_num(symbols) {
        let mut sym: TrieChar = symbols_get(symbols, i);
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
unsafe extern "C" fn da_relocate_base(
    mut d: *mut DArray,
    mut s: TrieIndex,
    mut new_base: TrieIndex,
) {
    let mut old_base: TrieIndex = 0;
    let mut symbols: *mut Symbols = 0 as *mut Symbols;
    let mut i: libc::c_int = 0;
    old_base = da_get_base(d, s);
    symbols = da_output_symbols(d, s);
    i = 0 as libc::c_int;
    while i < symbols_num(symbols) {
        let mut old_next: TrieIndex = 0;
        let mut new_next: TrieIndex = 0;
        let mut old_next_base: TrieIndex = 0;
        old_next = old_base + symbols_get(symbols, i) as libc::c_int;
        new_next = new_base + symbols_get(symbols, i) as libc::c_int;
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
    symbols_free(symbols);
    da_set_base(d, s, new_base);
}
unsafe extern "C" fn da_extend_pool(mut d: *mut DArray, mut to_index: TrieIndex) -> Bool {
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
unsafe extern "C" fn da_alloc_cell(mut d: *mut DArray, mut cell: TrieIndex) {
    let mut prev: TrieIndex = 0;
    let mut next: TrieIndex = 0;
    prev = -da_get_base(d, cell);
    next = -da_get_check(d, cell);
    da_set_check(d, prev, -next);
    da_set_base(d, next, -prev);
}
unsafe extern "C" fn da_free_cell(mut d: *mut DArray, mut cell: TrieIndex) {
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

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn serialize_int32_be_incr(buff: *mut *mut uint8, val: int32);
    fn file_read_int32(file: *mut FILE, o_val: *mut int32) -> Bool;
    fn file_write_int32(file: *mut FILE, val: int32) -> Bool;
    fn serialize_int16_be_incr(buff: *mut *mut uint8, val: int16);
    fn file_read_int16(file: *mut FILE, o_val: *mut int16) -> Bool;
    fn file_write_int16(file: *mut FILE, val: int16) -> Bool;
    fn file_read_chars(file: *mut FILE, buff: *mut libc::c_char, len: libc::c_int) -> Bool;
    fn file_write_chars(file: *mut FILE, buff: *const libc::c_char, len: libc::c_int) -> Bool;
    fn trie_char_strsize(str: *const TrieChar) -> size_t;
    fn trie_char_strlen(str: *const TrieChar) -> size_t;
    fn trie_char_strdup(str: *const TrieChar) -> *mut TrieChar;
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
pub type int16 = libc::c_short;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type TrieChar = libc::c_uchar;
pub type TrieIndex = int32;
pub type TrieData = int32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Tail {
    pub num_tails: TrieIndex,
    pub tails: *mut TailBlock,
    pub first_free: TrieIndex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TailBlock {
    pub next_free: TrieIndex,
    pub data: TrieData,
    pub suffix: *mut TrieChar,
}
pub type Tail = _Tail;
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
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const TRIE_CHAR_TERM: libc::c_int = '\0' as i32;
pub const TRIE_INDEX_ERROR: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const TAIL_SIGNATURE: libc::c_uint = 0xdffcdffc as libc::c_uint;
pub const TAIL_START_BLOCKNO: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn tail_new() -> *mut Tail {
    let mut t: *mut Tail = 0 as *mut Tail;
    t = malloc(::core::mem::size_of::<Tail>() as libc::c_ulong) as *mut Tail;
    if t.is_null() {
        return NULL as *mut Tail;
    }
    (*t).first_free = 0 as libc::c_int;
    (*t).num_tails = 0 as libc::c_int;
    (*t).tails = NULL as *mut TailBlock;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn tail_fread(mut file: *mut FILE) -> *mut Tail {
    let mut current_block: u64;
    let mut save_pos: libc::c_long = 0;
    let mut t: *mut Tail = 0 as *mut Tail;
    let mut i: TrieIndex = 0;
    let mut sig: uint32 = 0;
    save_pos = ftell(file);
    if !(file_read_int32(file, &mut sig as *mut uint32 as *mut int32) as u64 == 0
        || TAIL_SIGNATURE != sig)
    {
        t = malloc(::core::mem::size_of::<Tail>() as libc::c_ulong) as *mut Tail;
        if !t.is_null() {
            if !(file_read_int32(file, &mut (*t).first_free) as u64 == 0
                || file_read_int32(file, &mut (*t).num_tails) as u64 == 0)
            {
                if !((*t).num_tails as libc::c_ulong
                    > SIZE_MAX.wrapping_div(::core::mem::size_of::<TailBlock>() as libc::c_ulong))
                {
                    (*t).tails = malloc(
                        ((*t).num_tails as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<TailBlock>() as libc::c_ulong),
                    ) as *mut TailBlock;
                    if !((*t).tails).is_null() {
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < (*t).num_tails) {
                                current_block = 15904375183555213903;
                                break;
                            }
                            let mut length: int16 = 0;
                            if file_read_int32(
                                file,
                                &mut (*((*t).tails).offset(i as isize)).next_free,
                            ) as u64
                                == 0
                                || file_read_int32(
                                    file,
                                    &mut (*((*t).tails).offset(i as isize)).data,
                                ) as u64
                                    == 0
                                || file_read_int16(file, &mut length) as u64 == 0
                            {
                                current_block = 3120876903627340905;
                                break;
                            }
                            let ref mut fresh0 = (*((*t).tails).offset(i as isize)).suffix;
                            *fresh0 =
                                malloc((length as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                                    as *mut TrieChar;
                            if ((*((*t).tails).offset(i as isize)).suffix).is_null() {
                                current_block = 3120876903627340905;
                                break;
                            }
                            if length as libc::c_int > 0 as libc::c_int {
                                if file_read_chars(
                                    file,
                                    (*((*t).tails).offset(i as isize)).suffix as *mut libc::c_char,
                                    length as libc::c_int,
                                ) as u64
                                    == 0
                                {
                                    free(
                                        (*((*t).tails).offset(i as isize)).suffix
                                            as *mut libc::c_void,
                                    );
                                    current_block = 3120876903627340905;
                                    break;
                                }
                            }
                            *((*((*t).tails).offset(i as isize)).suffix).offset(length as isize) =
                                TRIE_CHAR_TERM as TrieChar;
                            i += 1;
                            i;
                        }
                        match current_block {
                            15904375183555213903 => return t,
                            _ => {
                                while i > 0 as libc::c_int {
                                    i -= 1;
                                    free(
                                        (*((*t).tails).offset(i as isize)).suffix
                                            as *mut libc::c_void,
                                    );
                                }
                                free((*t).tails as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
            free(t as *mut libc::c_void);
        }
    }
    fseek(file, save_pos, SEEK_SET);
    return NULL as *mut Tail;
}
#[no_mangle]
pub unsafe extern "C" fn tail_free(mut t: *mut Tail) {
    let mut i: TrieIndex = 0;
    if !((*t).tails).is_null() {
        i = 0 as libc::c_int;
        while i < (*t).num_tails {
            if !((*((*t).tails).offset(i as isize)).suffix).is_null() {
                free((*((*t).tails).offset(i as isize)).suffix as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        free((*t).tails as *mut libc::c_void);
    }
    free(t as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tail_fwrite(mut t: *const Tail, mut file: *mut FILE) -> libc::c_int {
    let mut i: TrieIndex = 0;
    if file_write_int32(file, TAIL_SIGNATURE as int32) as u64 == 0
        || file_write_int32(file, (*t).first_free) as u64 == 0
        || file_write_int32(file, (*t).num_tails) as u64 == 0
    {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*t).num_tails {
        let mut length: int16 = 0;
        if file_write_int32(file, (*((*t).tails).offset(i as isize)).next_free) as u64 == 0
            || file_write_int32(file, (*((*t).tails).offset(i as isize)).data) as u64 == 0
        {
            return -(1 as libc::c_int);
        }
        length = (if !((*((*t).tails).offset(i as isize)).suffix).is_null() {
            trie_char_strlen((*((*t).tails).offset(i as isize)).suffix)
        } else {
            0 as libc::c_int as size_t
        }) as int16;
        if file_write_int16(file, length) as u64 == 0 {
            return -(1 as libc::c_int);
        }
        if length as libc::c_int > 0 as libc::c_int
            && file_write_chars(
                file,
                (*((*t).tails).offset(i as isize)).suffix as *mut libc::c_char,
                length as libc::c_int,
            ) as u64
                == 0
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tail_get_serialized_size(mut t: *const Tail) -> size_t {
    let mut static_count: size_t = (::core::mem::size_of::<int32>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<TrieIndex>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<TrieIndex>() as libc::c_ulong);
    let mut dynamic_count: size_t = 0 as libc::c_uint as size_t;
    if (*t).num_tails > 0 as libc::c_int {
        let mut i: TrieIndex = 0 as libc::c_int;
        dynamic_count = (dynamic_count as libc::c_ulong).wrapping_add(
            (::core::mem::size_of::<TrieIndex>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<TrieData>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<int16>() as libc::c_ulong)
                .wrapping_mul((*t).num_tails as libc::c_ulong),
        ) as size_t as size_t;
        while i < (*t).num_tails {
            if !((*((*t).tails).offset(i as isize)).suffix).is_null() {
                dynamic_count = dynamic_count
                    .wrapping_add(trie_char_strsize((*((*t).tails).offset(i as isize)).suffix));
            }
            i += 1;
            i;
        }
    }
    return static_count.wrapping_add(dynamic_count);
}
#[no_mangle]
pub unsafe extern "C" fn tail_serialize(
    mut t: *const Tail,
    mut ptr: *mut *mut uint8,
) -> libc::c_int {
    let mut i: TrieIndex = 0;
    serialize_int32_be_incr(ptr, TAIL_SIGNATURE as int32);
    serialize_int32_be_incr(ptr, (*t).first_free);
    serialize_int32_be_incr(ptr, (*t).num_tails);
    i = 0 as libc::c_int;
    while i < (*t).num_tails {
        let mut length: int16 = 0;
        serialize_int32_be_incr(ptr, (*((*t).tails).offset(i as isize)).next_free);
        serialize_int32_be_incr(ptr, (*((*t).tails).offset(i as isize)).data);
        length = (if !((*((*t).tails).offset(i as isize)).suffix).is_null() {
            trie_char_strsize((*((*t).tails).offset(i as isize)).suffix)
        } else {
            0 as libc::c_int as size_t
        }) as int16;
        serialize_int16_be_incr(ptr, length);
        if length != 0 {
            memcpy(
                *ptr as *mut libc::c_void,
                (*((*t).tails).offset(i as isize)).suffix as *mut libc::c_char
                    as *const libc::c_void,
                length as libc::c_ulong,
            );
            *ptr = (*ptr).offset(length as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tail_get_suffix(
    mut t: *const Tail,
    mut index: TrieIndex,
) -> *const TrieChar {
    index -= TAIL_START_BLOCKNO;
    return if index < (*t).num_tails {
        (*((*t).tails).offset(index as isize)).suffix
    } else {
        NULL as *mut TrieChar
    };
}
#[no_mangle]
pub unsafe extern "C" fn tail_set_suffix(
    mut t: *mut Tail,
    mut index: TrieIndex,
    mut suffix: *const TrieChar,
) -> Bool {
    index -= TAIL_START_BLOCKNO;
    if index < (*t).num_tails {
        let mut tmp: *mut TrieChar = NULL as *mut TrieChar;
        if !suffix.is_null() {
            tmp = trie_char_strdup(suffix);
            if tmp.is_null() {
                return FALSE as Bool;
            }
        }
        if !((*((*t).tails).offset(index as isize)).suffix).is_null() {
            free((*((*t).tails).offset(index as isize)).suffix as *mut libc::c_void);
        }
        let ref mut fresh1 = (*((*t).tails).offset(index as isize)).suffix;
        *fresh1 = tmp;
        return TRUE as Bool;
    }
    return FALSE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn tail_add_suffix(
    mut t: *mut Tail,
    mut suffix: *const TrieChar,
) -> TrieIndex {
    let mut new_block: TrieIndex = 0;
    new_block = tail_alloc_block(t);
    if 0 as libc::c_int == new_block {
        return TRIE_INDEX_ERROR;
    }
    tail_set_suffix(t, new_block, suffix);
    return new_block;
}
unsafe extern "C" fn tail_alloc_block(mut t: *mut Tail) -> TrieIndex {
    let mut block: TrieIndex = 0;
    if 0 as libc::c_int != (*t).first_free {
        block = (*t).first_free;
        (*t).first_free = (*((*t).tails).offset(block as isize)).next_free;
    } else {
        let mut new_block: *mut libc::c_void = 0 as *mut libc::c_void;
        block = (*t).num_tails;
        new_block = realloc(
            (*t).tails as *mut libc::c_void,
            (((*t).num_tails + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TailBlock>() as libc::c_ulong),
        );
        if new_block.is_null() {
            return TRIE_INDEX_ERROR;
        }
        (*t).tails = new_block as *mut TailBlock;
        (*t).num_tails += 1;
        (*t).num_tails;
    }
    (*((*t).tails).offset(block as isize)).next_free = -(1 as libc::c_int);
    (*((*t).tails).offset(block as isize)).data = TRIE_DATA_ERROR;
    let ref mut fresh2 = (*((*t).tails).offset(block as isize)).suffix;
    *fresh2 = NULL as *mut TrieChar;
    return block + TAIL_START_BLOCKNO;
}
unsafe extern "C" fn tail_free_block(mut t: *mut Tail, mut block: TrieIndex) {
    let mut i: TrieIndex = 0;
    let mut j: TrieIndex = 0;
    block -= TAIL_START_BLOCKNO;
    if block >= (*t).num_tails {
        return;
    }
    (*((*t).tails).offset(block as isize)).data = TRIE_DATA_ERROR;
    if !((*((*t).tails).offset(block as isize)).suffix).is_null() {
        free((*((*t).tails).offset(block as isize)).suffix as *mut libc::c_void);
        let ref mut fresh3 = (*((*t).tails).offset(block as isize)).suffix;
        *fresh3 = NULL as *mut TrieChar;
    }
    j = 0 as libc::c_int;
    i = (*t).first_free;
    while i != 0 as libc::c_int && i < block {
        j = i;
        i = (*((*t).tails).offset(i as isize)).next_free;
    }
    (*((*t).tails).offset(block as isize)).next_free = i;
    if 0 as libc::c_int != j {
        (*((*t).tails).offset(j as isize)).next_free = block;
    } else {
        (*t).first_free = block;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tail_get_data(mut t: *const Tail, mut index: TrieIndex) -> TrieData {
    index -= TAIL_START_BLOCKNO;
    return if index < (*t).num_tails {
        (*((*t).tails).offset(index as isize)).data
    } else {
        TRIE_DATA_ERROR
    };
}
#[no_mangle]
pub unsafe extern "C" fn tail_set_data(
    mut t: *mut Tail,
    mut index: TrieIndex,
    mut data: TrieData,
) -> Bool {
    index -= TAIL_START_BLOCKNO;
    if index < (*t).num_tails {
        (*((*t).tails).offset(index as isize)).data = data;
        return TRUE as Bool;
    }
    return FALSE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn tail_delete(mut t: *mut Tail, mut index: TrieIndex) {
    tail_free_block(t, index);
}
#[no_mangle]
pub unsafe extern "C" fn tail_walk_str(
    mut t: *const Tail,
    mut s: TrieIndex,
    mut suffix_idx: *mut libc::c_short,
    mut str: *const TrieChar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut suffix: *const TrieChar = 0 as *const TrieChar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_short = 0;
    suffix = tail_get_suffix(t, s);
    if suffix.is_null() {
        return FALSE;
    }
    i = 0 as libc::c_int;
    j = *suffix_idx;
    while i < len {
        if *str.offset(i as isize) as libc::c_int != *suffix.offset(j as isize) as libc::c_int {
            break;
        }
        i += 1;
        i;
        if TRIE_CHAR_TERM == *suffix.offset(j as isize) as libc::c_int {
            break;
        }
        j += 1;
        j;
    }
    *suffix_idx = j;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn tail_walk_char(
    mut t: *const Tail,
    mut s: TrieIndex,
    mut suffix_idx: *mut libc::c_short,
    mut c: TrieChar,
) -> Bool {
    let mut suffix: *const TrieChar = 0 as *const TrieChar;
    let mut suffix_char: TrieChar = 0;
    suffix = tail_get_suffix(t, s);
    if suffix.is_null() {
        return FALSE as Bool;
    }
    suffix_char = *suffix.offset(*suffix_idx as isize);
    if suffix_char as libc::c_int == c as libc::c_int {
        if TRIE_CHAR_TERM != suffix_char as libc::c_int {
            *suffix_idx += 1;
            *suffix_idx;
        }
        return TRUE as Bool;
    }
    return FALSE as Bool;
}

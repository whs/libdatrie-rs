use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Trie;
    pub type _TrieState;
    pub type _TrieIterator;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn trie_free(trie: *mut Trie);
    fn trie_retrieve(trie: *const Trie, key: *const AlphaChar, o_data: *mut TrieData) -> Bool;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn trie_delete(trie: *mut Trie, key: *const AlphaChar) -> Bool;
    fn trie_root(trie: *const Trie) -> *mut TrieState;
    fn trie_state_free(s: *mut TrieState);
    fn trie_iterator_new(s: *mut TrieState) -> *mut TrieIterator;
    fn trie_iterator_free(iter: *mut TrieIterator);
    fn trie_iterator_next(iter: *mut TrieIterator) -> Bool;
    fn trie_iterator_get_key(iter: *const TrieIterator) -> *mut AlphaChar;
    fn trie_iterator_get_data(iter: *const TrieIterator) -> TrieData;
    fn msg_step(msg: *const libc::c_char);
    fn en_trie_new() -> *mut Trie;
    static mut dict_src: [DictRec; 0];
    fn dict_src_n_entries() -> libc::c_int;
    fn dict_src_get_data(key: *const AlphaChar) -> TrieData;
    fn dict_src_set_data(key: *const AlphaChar, data: TrieData) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn free(_: *mut libc::c_void);
    fn time(__timer: *mut time_t) -> time_t;
}
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type AlphaChar = uint32;
pub type TrieData = int32;
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type Trie = _Trie;
pub type TrieState = _TrieState;
pub type TrieIterator = _TrieIterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DictRec {
    pub key: *mut AlphaChar,
    pub data: TrieData,
}
pub type DictRec = _DictRec;
pub type wchar_t = libc::c_int;
pub type time_t = __time_t;
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const FALSE: libc::c_int = DA_FALSE as libc::c_int;
pub const TRIE_DATA_READ: libc::c_int = 2 as libc::c_int;
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
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe fn main_0() -> libc::c_int {
    let mut current_block: u64;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut dict_p: *mut DictRec = 0 as *mut DictRec;
    let mut trie_data: TrieData = 0;
    let mut is_failed: Bool = DA_FALSE;
    let mut n_entries: libc::c_int = 0;
    let mut n_dels: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut trie_root_state: *mut TrieState = 0 as *mut TrieState;
    let mut trie_it: *mut TrieIterator = 0 as *mut TrieIterator;
    msg_step(b"Preparing trie\0" as *const u8 as *const libc::c_char);
    test_trie = en_trie_new();
    if test_trie.is_null() {
        fprintf(
            stderr,
            b"Fail to create test trie\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        msg_step(b"Adding data to trie\0" as *const u8 as *const libc::c_char);
        dict_p = dict_src.as_mut_ptr();
        loop {
            if ((*dict_p).key).is_null() {
                current_block = 13536709405535804910;
                break;
            }
            if trie_store(test_trie, (*dict_p).key, (*dict_p).data) as u64 == 0 {
                printf(
                    b"Failed to add key '%ls', data %d.\n\0" as *const u8 as *const libc::c_char,
                    (*dict_p).key as *mut wchar_t,
                    (*dict_p).data,
                );
                current_block = 13792453122477194398;
                break;
            } else {
                dict_p = dict_p.offset(1);
                dict_p;
            }
        }
        match current_block {
            13536709405535804910 => {
                msg_step(b"Retrieving data from trie\0" as *const u8 as *const libc::c_char);
                is_failed = FALSE as Bool;
                dict_p = dict_src.as_mut_ptr();
                while !((*dict_p).key).is_null() {
                    if trie_retrieve(test_trie, (*dict_p).key, &mut trie_data) as u64 == 0 {
                        printf(
                            b"Failed to retrieve key '%ls'.\n\0" as *const u8
                                as *const libc::c_char,
                            (*dict_p).key as *mut wchar_t,
                        );
                        is_failed = TRUE as Bool;
                    }
                    if trie_data != (*dict_p).data {
                        printf(
                            b"Wrong data for key '%ls'; expected %d, got %d.\n\0" as *const u8
                                as *const libc::c_char,
                            (*dict_p).key as *mut wchar_t,
                            (*dict_p).data,
                            trie_data,
                        );
                        is_failed = TRUE as Bool;
                    }
                    dict_p = dict_p.offset(1);
                    dict_p;
                }
                if is_failed as u64 != 0 {
                    printf(
                        b"Trie store/retrieval test failed.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    msg_step(
                        b"Deleting some entries from trie\0" as *const u8 as *const libc::c_char,
                    );
                    n_entries = dict_src_n_entries();
                    srand(time(NULL as *mut time_t) as libc::c_uint);
                    n_dels = n_entries / 3 as libc::c_int + 1 as libc::c_int;
                    while n_dels > 0 as libc::c_int {
                        loop {
                            i = rand() % n_entries;
                            if !(TRIE_DATA_READ == (*dict_src.as_mut_ptr().offset(i as isize)).data)
                            {
                                break;
                            }
                        }
                        printf(
                            b"Deleting '%ls'\n\0" as *const u8 as *const libc::c_char,
                            (*dict_src.as_mut_ptr().offset(i as isize)).key as *mut wchar_t,
                        );
                        if trie_delete(test_trie, (*dict_src.as_mut_ptr().offset(i as isize)).key)
                            as u64
                            == 0
                        {
                            printf(
                                b"Failed to delete '%ls'\n\0" as *const u8 as *const libc::c_char,
                                (*dict_src.as_mut_ptr().offset(i as isize)).key as *mut wchar_t,
                            );
                            is_failed = TRUE as Bool;
                        }
                        (*dict_src.as_mut_ptr().offset(i as isize)).data = TRIE_DATA_READ;
                        n_dels -= 1;
                        n_dels;
                    }
                    if is_failed as u64 != 0 {
                        printf(
                            b"Trie deletion test failed.\n\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        msg_step(
                            b"Retrieving data from trie again after deletions\0" as *const u8
                                as *const libc::c_char,
                        );
                        dict_p = dict_src.as_mut_ptr();
                        while !((*dict_p).key).is_null() {
                            if !(TRIE_DATA_READ == (*dict_p).data) {
                                if trie_retrieve(test_trie, (*dict_p).key, &mut trie_data) as u64
                                    == 0
                                {
                                    printf(
                                        b"Failed to retrieve key '%ls'.\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*dict_p).key as *mut wchar_t,
                                    );
                                    is_failed = TRUE as Bool;
                                }
                                if trie_data != (*dict_p).data {
                                    printf(
                                        b"Wrong data for key '%ls'; expected %d, got %d.\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                        (*dict_p).key as *mut wchar_t,
                                        (*dict_p).data,
                                        trie_data,
                                    );
                                    is_failed = TRUE as Bool;
                                }
                            }
                            dict_p = dict_p.offset(1);
                            dict_p;
                        }
                        if is_failed as u64 != 0 {
                            printf(
                                b"Trie retrival-after-deletion test failed.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            msg_step(
                                b"Iterating trie contents after deletions\0" as *const u8
                                    as *const libc::c_char,
                            );
                            trie_root_state = trie_root(test_trie);
                            if trie_root_state.is_null() {
                                printf(
                                    b"Failed to get trie root state\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                trie_it = trie_iterator_new(trie_root_state);
                                if trie_it.is_null() {
                                    printf(
                                        b"Failed to get trie iterator\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    while trie_iterator_next(trie_it) as u64 != 0 {
                                        let mut key: *mut AlphaChar = 0 as *mut AlphaChar;
                                        let mut key_data: TrieData = 0;
                                        let mut src_data: TrieData = 0;
                                        key = trie_iterator_get_key(trie_it);
                                        if key.is_null() {
                                            printf(
                                                b"Failed to get key from trie iterator\n\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                            );
                                            is_failed = TRUE as Bool;
                                        } else {
                                            key_data = trie_iterator_get_data(trie_it);
                                            if TRIE_DATA_ERROR == key_data {
                                                printf(
                                                    b"Failed to get data from trie iterator for key '%ls'\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    key as *mut wchar_t,
                                                );
                                                is_failed = TRUE as Bool;
                                            }
                                            src_data = dict_src_get_data(key);
                                            if TRIE_DATA_ERROR == src_data {
                                                printf(
                                                    b"Extra entry in trie: key '%ls', data %d.\n\0"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    key as *mut wchar_t,
                                                    key_data,
                                                );
                                                is_failed = TRUE as Bool;
                                            } else if src_data != key_data {
                                                printf(
                                                    b"Data mismatch for: key '%ls', expected %d, got %d.\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    key as *mut wchar_t,
                                                    src_data,
                                                    key_data,
                                                );
                                                is_failed = TRUE as Bool;
                                            } else {
                                                dict_src_set_data(key, TRIE_DATA_READ);
                                            }
                                            free(key as *mut libc::c_void);
                                        }
                                    }
                                    dict_p = dict_src.as_mut_ptr();
                                    while !((*dict_p).key).is_null() {
                                        if (*dict_p).data != TRIE_DATA_READ {
                                            printf(
                                                b"Entry missed in trie: key '%ls', data %d.\n\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                (*dict_p).key as *mut wchar_t,
                                                (*dict_p).data,
                                            );
                                            is_failed = TRUE as Bool;
                                        }
                                        dict_p = dict_p.offset(1);
                                        dict_p;
                                    }
                                    if is_failed as u64 != 0 {
                                        printf(
                                            b"Errors found in trie iteration after deletions.\n\0"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        trie_iterator_free(trie_it);
                                    } else {
                                        trie_iterator_free(trie_it);
                                        trie_state_free(trie_root_state);
                                        trie_free(test_trie);
                                        return 0 as libc::c_int;
                                    }
                                }
                                trie_state_free(trie_root_state);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        trie_free(test_trie);
    }
    return 1 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

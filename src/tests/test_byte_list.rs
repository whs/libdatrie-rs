use ::libc;
extern "C" {
    pub type _AlphaMap;
    pub type _Trie;
    pub type _TrieState;
    pub type _TrieIterator;
    fn trie_new(alpha_map: *const AlphaMap) -> *mut Trie;
    fn trie_free(trie: *mut Trie);
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn trie_root(trie: *const Trie) -> *mut TrieState;
    fn trie_iterator_get_key(iter: *const TrieIterator) -> *mut AlphaChar;
    fn trie_iterator_next(iter: *mut TrieIterator) -> Bool;
    fn trie_iterator_free(iter: *mut TrieIterator);
    fn trie_iterator_new(s: *mut TrieState) -> *mut TrieIterator;
    fn trie_state_free(s: *mut TrieState);
    fn alpha_char_strcmp(str1: *const AlphaChar, str2: *const AlphaChar) -> libc::c_int;
    fn alpha_map_add_range(
        alpha_map: *mut AlphaMap,
        begin: AlphaChar,
        end: AlphaChar,
    ) -> libc::c_int;
    fn alpha_map_free(alpha_map: *mut AlphaMap);
    fn alpha_map_new() -> *mut AlphaMap;
    fn trie_iterator_get_data(iter: *const TrieIterator) -> TrieData;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn msg_step(msg: *const libc::c_char);
    fn free(_: *mut libc::c_void);
}
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type AlphaChar = uint32;
pub type TrieData = int32;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type AlphaMap = _AlphaMap;
pub type Trie = _Trie;
pub type TrieState = _TrieState;
pub type TrieIterator = _TrieIterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DictEntry {
    pub key: [AlphaChar; 4],
    pub data: TrieData,
    pub is_checked: libc::c_int,
}
pub type DictEntry = _DictEntry;
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
static mut Source: [DictEntry; 2] = [
    {
        let mut init = _DictEntry {
            key: [
                '1' as i32 as AlphaChar,
                '2' as i32 as AlphaChar,
                0 as libc::c_int as AlphaChar,
                0,
            ],
            data: 1 as libc::c_int,
            is_checked: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = _DictEntry {
            key: [
                '1' as i32 as AlphaChar,
                '2' as i32 as AlphaChar,
                '3' as i32 as AlphaChar,
                0 as libc::c_int as AlphaChar,
            ],
            data: 2 as libc::c_int,
            is_checked: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn dump_key_data(mut key: *const AlphaChar, mut data: TrieData) {
    let mut p: *const AlphaChar = 0 as *const AlphaChar;
    printf(b"[\0" as *const u8 as *const libc::c_char);
    p = key;
    while *p != 0 {
        if p != key {
            printf(b", \0" as *const u8 as *const libc::c_char);
        }
        printf(b"%04x\0" as *const u8 as *const libc::c_char, *p);
        p = p.offset(1);
        p;
    }
    printf(b"] : %d\n\0" as *const u8 as *const libc::c_char, data);
}
unsafe extern "C" fn dump_entry(mut iter: *const TrieIterator) {
    let mut key: *mut AlphaChar = trie_iterator_get_key(iter);
    dump_key_data(key, trie_iterator_get_data(iter));
    free(key as *mut libc::c_void);
}
unsafe extern "C" fn validate_entry(mut iter: *const TrieIterator) -> libc::c_int {
    let mut key: *mut AlphaChar = trie_iterator_get_key(iter);
    let mut data: TrieData = trie_iterator_get_data(iter);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[DictEntry; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<DictEntry>() as libc::c_ulong)
    {
        if alpha_char_strcmp((Source[i as usize].key).as_mut_ptr(), key) == 0 as libc::c_int
            && Source[i as usize].data == data
        {
            Source[i as usize].is_checked = 1 as libc::c_int;
            free(key as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    free(key as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_all_checked() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[DictEntry; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<DictEntry>() as libc::c_ulong)
    {
        if Source[i as usize].is_checked == 0 {
            printf(b"Not visited Source entry: \0" as *const u8 as *const libc::c_char);
            dump_key_data(
                (Source[i as usize].key).as_mut_ptr(),
                Source[i as usize].data,
            );
            ret = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe fn main_0() -> libc::c_int {
    let mut current_block: u64;
    let mut alpha_map: *mut AlphaMap = 0 as *mut AlphaMap;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut i: libc::c_int = 0;
    let mut root: *mut TrieState = 0 as *mut TrieState;
    let mut iter: *mut TrieIterator = 0 as *mut TrieIterator;
    let mut ret: libc::c_int = 0 as libc::c_int;
    msg_step(b"Preparing alpha map\0" as *const u8 as *const libc::c_char);
    alpha_map = alpha_map_new();
    if alpha_map.is_null() {
        printf(b"Fail to allocate alpha map\n\0" as *const u8 as *const libc::c_char);
    } else {
        if alpha_map_add_range(
            alpha_map,
            0 as libc::c_int as AlphaChar,
            0xff as libc::c_int as AlphaChar,
        ) != 0 as libc::c_int
        {
            printf(b"Fail to add full alpha map range\n\0" as *const u8 as *const libc::c_char);
        } else {
            msg_step(b"Preparing trie\0" as *const u8 as *const libc::c_char);
            test_trie = trie_new(alpha_map);
            alpha_map_free(alpha_map);
            if test_trie.is_null() {
                printf(b"Fail to create test trie\n\0" as *const u8 as *const libc::c_char);
            } else {
                msg_step(b"Storing entries to test trie\0" as *const u8 as *const libc::c_char);
                i = 0 as libc::c_int;
                loop {
                    if !((i as libc::c_ulong)
                        < (::core::mem::size_of::<[DictEntry; 2]>() as libc::c_ulong)
                            .wrapping_div(::core::mem::size_of::<DictEntry>() as libc::c_ulong))
                    {
                        current_block = 15976848397966268834;
                        break;
                    }
                    if trie_store(
                        test_trie,
                        (Source[i as usize].key).as_mut_ptr(),
                        Source[i as usize].data,
                    ) as u64
                        == 0
                    {
                        printf(
                            b"Fail to store entry %d to test trie:\n\0" as *const u8
                                as *const libc::c_char,
                            i,
                        );
                        dump_key_data(
                            (Source[i as usize].key).as_mut_ptr(),
                            Source[i as usize].data,
                        );
                        current_block = 2156429523407074221;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                match current_block {
                    2156429523407074221 => {
                        trie_free(test_trie);
                    }
                    _ => {
                        msg_step(b"Iterating trie\0" as *const u8 as *const libc::c_char);
                        root = trie_root(test_trie);
                        iter = trie_iterator_new(root);
                        while trie_iterator_next(iter) as u64 != 0 {
                            if validate_entry(iter) == 0 {
                                printf(
                                    b"Fail to validate trie entry:\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                dump_entry(iter);
                                ret = 1 as libc::c_int;
                            }
                        }
                        if is_all_checked() == 0 {
                            ret = 1 as libc::c_int;
                        }
                        trie_iterator_free(iter);
                        trie_state_free(root);
                        msg_step(b"Freeing test trie\0" as *const u8 as *const libc::c_char);
                        trie_free(test_trie);
                        return ret;
                    }
                }
            }
        }
        alpha_map_free(alpha_map);
    }
    return 1 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

use ::libc;
extern "C" {
    pub type _AlphaMap;
    pub type _Trie;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn alpha_map_add_range(
        alpha_map: *mut AlphaMap,
        begin: AlphaChar,
        end: AlphaChar,
    ) -> libc::c_int;
    fn alpha_map_new() -> *mut AlphaMap;
    fn alpha_map_free(alpha_map: *mut AlphaMap);
    fn trie_new(alpha_map: *const AlphaMap) -> *mut Trie;
    fn alpha_char_strcmp(str1: *const AlphaChar, str2: *const AlphaChar) -> libc::c_int;
}
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type AlphaChar = uint32;
pub type TrieData = int32;
pub type AlphaMap = _AlphaMap;
pub type Trie = _Trie;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DictRec {
    pub key: *mut AlphaChar,
    pub data: TrieData,
}
pub type DictRec = _DictRec;
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const TRIE_DATA_UNREAD: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn msg_step(mut msg: *const libc::c_char) {
    printf(b"=> %s...\n\0" as *const u8 as *const libc::c_char, msg);
}
unsafe extern "C" fn en_alpha_map_new() -> *mut AlphaMap {
    let mut en_map: *mut AlphaMap = 0 as *mut AlphaMap;
    en_map = alpha_map_new();
    if !en_map.is_null() {
        if alpha_map_add_range(
            en_map,
            0x61 as libc::c_int as AlphaChar,
            0x7a as libc::c_int as AlphaChar,
        ) != 0 as libc::c_int
        {
            alpha_map_free(en_map);
        } else {
            return en_map;
        }
    }
    return NULL as *mut AlphaMap;
}
#[no_mangle]
pub unsafe extern "C" fn en_trie_new() -> *mut Trie {
    let mut en_map: *mut AlphaMap = 0 as *mut AlphaMap;
    let mut en_trie: *mut Trie = 0 as *mut Trie;
    en_map = en_alpha_map_new();
    if !en_map.is_null() {
        en_trie = trie_new(en_map);
        if en_trie.is_null() {
            alpha_map_free(en_map);
        } else {
            alpha_map_free(en_map);
            return en_trie;
        }
    }
    return NULL as *mut Trie;
}
#[no_mangle]
pub static mut dict_src: [DictRec; 40] = unsafe {
    [
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b"a\0\0\0\0\0\0\0"))
                    .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 28], &[libc::c_int; 7]>(
                    b"a\0\0\0b\0\0\0a\0\0\0c\0\0\0u\0\0\0s\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"a\0\0\0b\0\0\0a\0\0\0n\0\0\0d\0\0\0o\0\0\0n\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 36], &[libc::c_int; 9]>(
                    b"a\0\0\0c\0\0\0c\0\0\0i\0\0\0d\0\0\0e\0\0\0n\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 36], &[libc::c_int; 9]>(
                    b"a\0\0\0c\0\0\0c\0\0\0r\0\0\0e\0\0\0d\0\0\0i\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 40], &[libc::c_int; 10]>(
                    b"a\0\0\0l\0\0\0g\0\0\0o\0\0\0r\0\0\0i\0\0\0t\0\0\0h\0\0\0m\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"a\0\0\0m\0\0\0m\0\0\0o\0\0\0n\0\0\0i\0\0\0a\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"a\0\0\0n\0\0\0g\0\0\0e\0\0\0l\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"a\0\0\0n\0\0\0g\0\0\0l\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"a\0\0\0z\0\0\0u\0\0\0r\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"b\0\0\0a\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"b\0\0\0e\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"b\0\0\0e\0\0\0s\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"h\0\0\0o\0\0\0m\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"h\0\0\0o\0\0\0u\0\0\0s\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"h\0\0\0u\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"k\0\0\0i\0\0\0n\0\0\0g\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"k\0\0\0i\0\0\0t\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"n\0\0\0a\0\0\0m\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"n\0\0\0e\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"n\0\0\0e\0\0\0t\0\0\0w\0\0\0o\0\0\0r\0\0\0k\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"n\0\0\0u\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 36], &[libc::c_int; 9]>(
                    b"n\0\0\0u\0\0\0t\0\0\0s\0\0\0h\0\0\0e\0\0\0l\0\0\0l\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"q\0\0\0u\0\0\0a\0\0\0l\0\0\0i\0\0\0t\0\0\0y\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"q\0\0\0u\0\0\0a\0\0\0n\0\0\0t\0\0\0u\0\0\0m\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 36], &[libc::c_int; 9]>(
                    b"q\0\0\0u\0\0\0a\0\0\0n\0\0\0t\0\0\0i\0\0\0t\0\0\0y\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 28], &[libc::c_int; 7]>(
                    b"q\0\0\0u\0\0\0a\0\0\0r\0\0\0t\0\0\0z\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"q\0\0\0u\0\0\0i\0\0\0c\0\0\0k\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"q\0\0\0u\0\0\0i\0\0\0z\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"r\0\0\0u\0\0\0n\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"t\0\0\0a\0\0\0p\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"t\0\0\0e\0\0\0s\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"w\0\0\0h\0\0\0a\0\0\0t\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"w\0\0\0h\0\0\0e\0\0\0n\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"w\0\0\0h\0\0\0e\0\0\0r\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"w\0\0\0h\0\0\0i\0\0\0c\0\0\0h\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"w\0\0\0h\0\0\0o\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                    b"w\0\0\0h\0\0\0y\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"z\0\0\0e\0\0\0b\0\0\0r\0\0\0a\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: NULL as *mut libc::c_void as *mut AlphaChar,
                data: TRIE_DATA_ERROR,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn dict_src_n_entries() -> libc::c_int {
    return (::core::mem::size_of::<[DictRec; 40]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<DictRec>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dict_src_get_data(mut key: *const AlphaChar) -> TrieData {
    let mut dict_p: *const DictRec = 0 as *const DictRec;
    dict_p = dict_src.as_mut_ptr();
    while !((*dict_p).key).is_null() {
        if alpha_char_strcmp((*dict_p).key, key) == 0 as libc::c_int {
            return (*dict_p).data;
        }
        dict_p = dict_p.offset(1);
        dict_p;
    }
    return TRIE_DATA_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn dict_src_set_data(
    mut key: *const AlphaChar,
    mut data: TrieData,
) -> libc::c_int {
    let mut dict_p: *mut DictRec = 0 as *mut DictRec;
    dict_p = dict_src.as_mut_ptr();
    while !((*dict_p).key).is_null() {
        if alpha_char_strcmp((*dict_p).key, key) == 0 as libc::c_int {
            (*dict_p).data = data;
            return 0 as libc::c_int;
        }
        dict_p = dict_p.offset(1);
        dict_p;
    }
    return -(1 as libc::c_int);
}

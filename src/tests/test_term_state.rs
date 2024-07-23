use ::libc;
extern "C" {
    pub type _Trie;
    pub type _TrieState;
    fn trie_free(trie: *mut Trie);
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn trie_state_get_data(s: *const TrieState) -> TrieData;
    fn trie_state_walk(s: *mut TrieState, c: AlphaChar) -> Bool;
    fn trie_state_free(s: *mut TrieState);
    fn trie_root(trie: *const Trie) -> *mut TrieState;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn msg_step(msg: *const libc::c_char);
    fn en_trie_new() -> *mut Trie;
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
pub type Trie = _Trie;
pub type TrieState = _TrieState;
pub const FALSE: libc::c_int = DA_FALSE as libc::c_int;
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
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
unsafe fn main_0() -> libc::c_int {
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut trie_state: *mut TrieState = 0 as *mut TrieState;
    let mut data: TrieData = 0;
    let mut is_failed: Bool = DA_FALSE;
    msg_step(b"Preparing trie\0" as *const u8 as *const libc::c_char);
    test_trie = en_trie_new();
    if test_trie.is_null() {
        printf(b"Fail to create test trie\n\0" as *const u8 as *const libc::c_char);
    } else {
        msg_step(b"Populating trie with test set\0" as *const u8 as *const libc::c_char);
        if trie_store(
            test_trie,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_int; 3]>(b"a\0\0\0b\0\0\0\0\0\0\0"))
                .as_ptr() as *mut AlphaChar,
            1 as libc::c_int,
        ) as u64
            == 0
        {
            printf(b"Failed to add key 'ab', data 1.\n\0" as *const u8 as *const libc::c_char);
        } else if trie_store(
            test_trie,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_int; 4]>(
                b"a\0\0\0b\0\0\0c\0\0\0\0\0\0\0",
            ))
            .as_ptr() as *mut AlphaChar,
            2 as libc::c_int,
        ) as u64
            == 0
        {
            printf(b"Failed to add key 'abc', data 2.\n\0" as *const u8 as *const libc::c_char);
        } else {
            is_failed = FALSE as Bool;
            msg_step(b"Preparing root state\0" as *const u8 as *const libc::c_char);
            trie_state = trie_root(test_trie);
            if trie_state.is_null() {
                printf(b"Failed to get trie root state\n\0" as *const u8 as *const libc::c_char);
            } else {
                msg_step(b"Try walking from root with 'a'\0" as *const u8 as *const libc::c_char);
                if trie_state_walk(trie_state, 'a' as i32 as AlphaChar) as u64 == 0 {
                    printf(
                        b"Failed to walk from root with 'a'.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    is_failed = TRUE as Bool;
                }
                data = trie_state_get_data(trie_state);
                if data != TRIE_DATA_ERROR {
                    printf(
                        b"Retrieved data at 'a' is %d, not %d.\n\0" as *const u8
                            as *const libc::c_char,
                        data,
                        TRIE_DATA_ERROR,
                    );
                    is_failed = TRUE as Bool;
                }
                msg_step(b"Try walking further with 'b'\0" as *const u8 as *const libc::c_char);
                if trie_state_walk(trie_state, 'b' as i32 as AlphaChar) as u64 == 0 {
                    printf(
                        b"Failed to continue walking with 'b'.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    is_failed = TRUE as Bool;
                }
                data = trie_state_get_data(trie_state);
                if data != 1 as libc::c_int {
                    printf(
                        b"Retrieved data for key 'ab' is %d, not 1.\n\0" as *const u8
                            as *const libc::c_char,
                        data,
                    );
                    is_failed = TRUE as Bool;
                }
                msg_step(b"Try walking further with 'c'\0" as *const u8 as *const libc::c_char);
                if trie_state_walk(trie_state, 'c' as i32 as AlphaChar) as u64 == 0 {
                    printf(
                        b"Failed to continue walking with 'c'.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    is_failed = TRUE as Bool;
                }
                data = trie_state_get_data(trie_state);
                if data != 2 as libc::c_int {
                    printf(
                        b"Retrieved data for key 'abc' is %d, not 2.\n\0" as *const u8
                            as *const libc::c_char,
                        data,
                    );
                    is_failed = TRUE as Bool;
                }
                trie_state_free(trie_state);
                if is_failed as u64 != 0 {
                    printf(
                        b"Errors found in terminal state data retrieval.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    trie_free(test_trie);
                    return 0 as libc::c_int;
                }
            }
        }
        trie_free(test_trie);
    }
    return 1 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

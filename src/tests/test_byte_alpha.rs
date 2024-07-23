use ::libc;
extern "C" {
    pub type _AlphaMap;
    pub type _Trie;
    fn trie_new(alpha_map: *const AlphaMap) -> *mut Trie;
    fn trie_free(trie: *mut Trie);
    fn trie_retrieve(trie: *const Trie, key: *const AlphaChar, o_data: *mut TrieData) -> Bool;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn alpha_map_new() -> *mut AlphaMap;
    fn alpha_map_free(alpha_map: *mut AlphaMap);
    fn alpha_map_add_range(
        alpha_map: *mut AlphaMap,
        begin: AlphaChar,
        end: AlphaChar,
    ) -> libc::c_int;
    fn msg_step(msg: *const libc::c_char);
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
pub const TEST_DATA: libc::c_int = 255 as libc::c_int;
unsafe fn main_0() -> libc::c_int {
    let mut alpha_map: *mut AlphaMap = 0 as *mut AlphaMap;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut key: [AlphaChar; 3] = [0; 3];
    let mut data: TrieData = 0;
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
                msg_step(b"Storing key to test trie\0" as *const u8 as *const libc::c_char);
                key[0 as libc::c_int as usize] = 0xff as libc::c_int as AlphaChar;
                key[1 as libc::c_int as usize] = 0xff as libc::c_int as AlphaChar;
                key[2 as libc::c_int as usize] = 0 as libc::c_int as AlphaChar;
                if trie_store(test_trie, key.as_mut_ptr(), TEST_DATA) as u64 == 0 {
                    printf(
                        b"Fail to store key to test trie\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    msg_step(
                        b"Retrieving data from test trie\0" as *const u8 as *const libc::c_char,
                    );
                    if trie_retrieve(test_trie, key.as_mut_ptr(), &mut data) as u64 == 0 {
                        printf(
                            b"Fail to retrieve key from test trie\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if TEST_DATA != data {
                        printf(
                            b"Retrieved data = %d, not %d\n\0" as *const u8 as *const libc::c_char,
                            data,
                            TEST_DATA,
                        );
                    } else {
                        msg_step(b"Freeing test trie\0" as *const u8 as *const libc::c_char);
                        trie_free(test_trie);
                        return 0 as libc::c_int;
                    }
                }
                trie_free(test_trie);
            }
        }
        alpha_map_free(alpha_map);
    }
    return 1 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

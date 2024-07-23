use ::libc;
extern "C" {
    pub type _Trie;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn trie_new_from_file(path: *const libc::c_char) -> *mut Trie;
    fn trie_free(trie: *mut Trie);
    fn trie_save(trie: *mut Trie, path: *const libc::c_char) -> libc::c_int;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn trie_enumerate(
        trie: *const Trie,
        enum_func: TrieEnumFunc,
        user_data: *mut libc::c_void,
    ) -> Bool;
    fn msg_step(msg: *const libc::c_char);
    fn en_trie_new() -> *mut Trie;
    static mut dict_src: [DictRec; 0];
    fn dict_src_get_data(key: *const AlphaChar) -> TrieData;
    fn dict_src_set_data(key: *const AlphaChar, data: TrieData) -> libc::c_int;
}
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type AlphaChar = uint32;
pub type TrieData = int32;
pub type Trie = _Trie;
pub type TrieEnumFunc =
    Option<unsafe extern "C" fn(*const AlphaChar, TrieData, *mut libc::c_void) -> Bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DictRec {
    pub key: *mut AlphaChar,
    pub data: TrieData,
}
pub type DictRec = _DictRec;
pub type wchar_t = libc::c_int;
pub const FALSE: libc::c_int = DA_FALSE as libc::c_int;
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
pub const TRIE_DATA_READ: libc::c_int = 2 as libc::c_int;
pub const TRIE_FILENAME: [libc::c_char; 9] =
    unsafe { *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"test.tri\0") };
unsafe extern "C" fn trie_enum_mark_rec(
    mut key: *const AlphaChar,
    mut key_data: TrieData,
    mut user_data: *mut libc::c_void,
) -> Bool {
    let mut is_failed: *mut Bool = user_data as *mut Bool;
    let mut src_data: TrieData = 0;
    src_data = dict_src_get_data(key);
    if TRIE_DATA_ERROR == src_data {
        printf(
            b"Extra entry in file: key '%ls', data %d.\n\0" as *const u8 as *const libc::c_char,
            key as *mut wchar_t,
            key_data,
        );
        *is_failed = TRUE as Bool;
    } else if src_data != key_data {
        printf(
            b"Data mismatch for: key '%ls', expected %d, got %d.\n\0" as *const u8
                as *const libc::c_char,
            key as *mut wchar_t,
            src_data,
            key_data,
        );
        *is_failed = TRUE as Bool;
    } else {
        dict_src_set_data(key, TRIE_DATA_READ);
    }
    return TRUE as Bool;
}
unsafe fn main_0() -> libc::c_int {
    let mut current_block: u64;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut dict_p: *mut DictRec = 0 as *mut DictRec;
    let mut is_failed: Bool = DA_FALSE;
    msg_step(b"Preparing trie\0" as *const u8 as *const libc::c_char);
    test_trie = en_trie_new();
    if test_trie.is_null() {
        printf(b"Failed to allocate test trie.\n\0" as *const u8 as *const libc::c_char);
    } else {
        dict_p = dict_src.as_mut_ptr();
        loop {
            if ((*dict_p).key).is_null() {
                current_block = 13513818773234778473;
                break;
            }
            if trie_store(test_trie, (*dict_p).key, (*dict_p).data) as u64 == 0 {
                printf(
                    b"Failed to add key '%ls', data %d.\n\0" as *const u8 as *const libc::c_char,
                    (*dict_p).key as *mut wchar_t,
                    (*dict_p).data,
                );
                current_block = 18333008207481592075;
                break;
            } else {
                dict_p = dict_p.offset(1);
                dict_p;
            }
        }
        match current_block {
            13513818773234778473 => {
                msg_step(b"Saving trie to file\0" as *const u8 as *const libc::c_char);
                remove(TRIE_FILENAME.as_ptr());
                if trie_save(test_trie, TRIE_FILENAME.as_ptr()) != 0 as libc::c_int {
                    printf(
                        b"Failed to save trie to file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        TRIE_FILENAME.as_ptr(),
                    );
                } else {
                    trie_free(test_trie);
                    msg_step(
                        b"Reloading trie from the saved file\0" as *const u8 as *const libc::c_char,
                    );
                    test_trie = trie_new_from_file(TRIE_FILENAME.as_ptr());
                    if test_trie.is_null() {
                        printf(
                            b"Failed to reload saved trie from '%s'.\n\0" as *const u8
                                as *const libc::c_char,
                            TRIE_FILENAME.as_ptr(),
                        );
                    } else {
                        msg_step(b"Checking trie contents\0" as *const u8 as *const libc::c_char);
                        is_failed = FALSE as Bool;
                        if trie_enumerate(
                            test_trie,
                            Some(
                                trie_enum_mark_rec
                                    as unsafe extern "C" fn(
                                        *const AlphaChar,
                                        TrieData,
                                        *mut libc::c_void,
                                    )
                                        -> Bool,
                            ),
                            &mut is_failed as *mut Bool as *mut libc::c_void,
                        ) as u64
                            == 0
                        {
                            printf(
                                b"Failed to enumerate trie file contents.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            dict_p = dict_src.as_mut_ptr();
                            while !((*dict_p).key).is_null() {
                                if (*dict_p).data != TRIE_DATA_READ {
                                    printf(
                                        b"Entry missed in file: key '%ls', data %d.\n\0"
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
                                    b"Errors found in trie saved contents.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                remove(TRIE_FILENAME.as_ptr());
                                trie_free(test_trie);
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    remove(TRIE_FILENAME.as_ptr());
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

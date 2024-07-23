use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Trie;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn trie_free(trie: *mut Trie);
    fn trie_retrieve(trie: *const Trie, key: *const AlphaChar, o_data: *mut TrieData) -> Bool;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn msg_step(msg: *const libc::c_char);
    fn en_trie_new() -> *mut Trie;
    static mut dict_src: [DictRec; 0];
}
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type AlphaChar = uint32;
pub type TrieData = int32;
pub type size_t = libc::c_ulong;
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
pub type Trie = _Trie;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DictRec {
    pub key: *mut AlphaChar,
    pub data: TrieData,
}
pub type DictRec = _DictRec;
pub type wchar_t = libc::c_int;
pub const FALSE: libc::c_int = DA_FALSE as libc::c_int;
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
pub const TRIE_DATA_UNREAD: libc::c_int = 1 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut nonalpha_src: [*const AlphaChar; 3] = unsafe {
    [
        (*::core::mem::transmute::<&[u8; 28], &[libc::c_int; 7]>(
            b"a\0\0\x006\0\0\0a\0\0\0c\0\0\0u\0\0\0s\0\0\0\0\0\0\0",
        ))
        .as_ptr() as *mut AlphaChar as *const AlphaChar,
        (*::core::mem::transmute::<&[u8; 28], &[libc::c_int; 7]>(
            b"a\0\0\x005\0\0\0a\0\0\0c\0\0\0u\0\0\0s\0\0\0\0\0\0\0",
        ))
        .as_ptr() as *mut AlphaChar as *const AlphaChar,
        NULL as *const AlphaChar,
    ]
};
unsafe fn main_0() -> libc::c_int {
    let mut current_block: u64;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut dict_p: *mut DictRec = 0 as *mut DictRec;
    let mut nonalpha_key: *mut *const AlphaChar = 0 as *mut *const AlphaChar;
    let mut trie_data: TrieData = 0;
    let mut is_fail: Bool = DA_FALSE;
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
                current_block = 1917311967535052937;
                break;
            }
            if trie_store(test_trie, (*dict_p).key, (*dict_p).data) as u64 == 0 {
                printf(
                    b"Failed to add key '%ls', data %d.\n\0" as *const u8 as *const libc::c_char,
                    (*dict_p).key as *mut wchar_t,
                    (*dict_p).data,
                );
                current_block = 8352418386514959403;
                break;
            } else {
                dict_p = dict_p.offset(1);
                dict_p;
            }
        }
        match current_block {
            1917311967535052937 => {
                is_fail = FALSE as Bool;
                nonalpha_key = nonalpha_src.as_mut_ptr();
                while !(*nonalpha_key).is_null() {
                    if trie_retrieve(test_trie, *nonalpha_key, &mut trie_data) as u64 != 0 {
                        printf(
                            b"False duplication on key '%ls', with existing data %d.\n\0"
                                as *const u8 as *const libc::c_char,
                            *nonalpha_key as *mut wchar_t,
                            trie_data,
                        );
                        is_fail = TRUE as Bool;
                    }
                    if trie_store(test_trie, *nonalpha_key, TRIE_DATA_UNREAD) as u64 != 0 {
                        printf(
                            b"Wrongly added key '%ls' containing non-alphanet char\n\0" as *const u8
                                as *const libc::c_char,
                            *nonalpha_key as *mut wchar_t,
                        );
                        is_fail = TRUE as Bool;
                    }
                    nonalpha_key = nonalpha_key.offset(1);
                    nonalpha_key;
                }
                if !(is_fail as u64 != 0) {
                    trie_free(test_trie);
                    return 0 as libc::c_int;
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

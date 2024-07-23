use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Trie;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn trie_free(trie: *mut Trie);
    fn trie_get_serialized_size(trie: *mut Trie) -> size_t;
    fn trie_serialize(trie: *mut Trie, ptr: *mut uint8);
    fn trie_save(trie: *mut Trie, path: *const libc::c_char) -> libc::c_int;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn msg_step(msg: *const libc::c_char);
    fn en_trie_new() -> *mut Trie;
    static mut dict_src: [DictRec; 0];
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
}
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint8 = libc::c_uchar;
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
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
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
pub const TRIE_FILENAME: [libc::c_char; 9] =
    unsafe { *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"test.tri\0") };
unsafe fn main_0() -> libc::c_int {
    let mut trieFileData: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut file_size: size_t = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut size: size_t = 0;
    let mut trieSerializedData: *mut uint8 = 0 as *mut uint8;
    let mut current_block: u64;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut dict_p: *mut DictRec = 0 as *mut DictRec;
    msg_step(b"Preparing trie\0" as *const u8 as *const libc::c_char);
    test_trie = en_trie_new();
    if test_trie.is_null() {
        printf(b"Failed to allocate test trie.\n\0" as *const u8 as *const libc::c_char);
    } else {
        dict_p = dict_src.as_mut_ptr();
        loop {
            if ((*dict_p).key).is_null() {
                current_block = 17216689946888361452;
                break;
            }
            if trie_store(test_trie, (*dict_p).key, (*dict_p).data) as u64 == 0 {
                printf(
                    b"Failed to add key '%ls', data %d.\n\0" as *const u8 as *const libc::c_char,
                    (*dict_p).key as *mut wchar_t,
                    (*dict_p).data,
                );
                current_block = 14225411149259281079;
                break;
            } else {
                dict_p = dict_p.offset(1);
                dict_p;
            }
        }
        match current_block {
            17216689946888361452 => {
                msg_step(b"Saving trie to file\0" as *const u8 as *const libc::c_char);
                remove(TRIE_FILENAME.as_ptr());
                if trie_save(test_trie, TRIE_FILENAME.as_ptr()) != 0 as libc::c_int {
                    printf(
                        b"Failed to save trie to file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        TRIE_FILENAME.as_ptr(),
                    );
                } else {
                    msg_step(b"Getting serialized trie size\0" as *const u8 as *const libc::c_char);
                    size = trie_get_serialized_size(test_trie);
                    printf(
                        b"serialized trie size %lu\n\0" as *const u8 as *const libc::c_char,
                        size,
                    );
                    msg_step(b"Allocating\0" as *const u8 as *const libc::c_char);
                    trieSerializedData = malloc(size) as *mut uint8;
                    if trieSerializedData.is_null() {
                        printf(
                            b"Failed to allocate trieSerializedData.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        printf(
                            b"allocated %p\n\0" as *const u8 as *const libc::c_char,
                            trieSerializedData,
                        );
                        msg_step(b"Serializing\0" as *const u8 as *const libc::c_char);
                        trie_serialize(test_trie, trieSerializedData);
                        msg_step(b"Serialized\0" as *const u8 as *const libc::c_char);
                        f = fopen(
                            TRIE_FILENAME.as_ptr(),
                            b"rb\0" as *const u8 as *const libc::c_char,
                        );
                        if f.is_null() {
                            printf(
                                b"Failed to reopen trie file test.tri.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            fseek(f, 0 as libc::c_int as libc::c_long, SEEK_END);
                            file_size = ftell(f) as size_t;
                            fseek(f, 0 as libc::c_int as libc::c_long, SEEK_SET);
                            if size != file_size {
                                printf(
                                    b"Trie serialized data doesn't match size of the file.\n\0"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                trieFileData = malloc(size) as *mut libc::c_uchar;
                                if trieFileData.is_null() {
                                    printf(
                                        b"Failed to allocate trieFileData.\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    if fread(
                                        trieFileData as *mut libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                        size,
                                        f,
                                    ) != size
                                    {
                                        printf(
                                            b"Failed to read back the serialized trie file.\n\0"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                    } else if memcmp(
                                        trieSerializedData as *const libc::c_void,
                                        trieFileData as *const libc::c_void,
                                        size,
                                    ) != 0 as libc::c_int
                                    {
                                        printf(
                                            b"Trie serialized data doesn't match contents of the file.\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    } else {
                                        printf(b"PASS!\n\0" as *const u8 as *const libc::c_char);
                                        free(trieFileData as *mut libc::c_void);
                                        fclose(f);
                                        free(trieSerializedData as *mut libc::c_void);
                                        remove(TRIE_FILENAME.as_ptr());
                                        trie_free(test_trie);
                                        return 0 as libc::c_int;
                                    }
                                    free(trieFileData as *mut libc::c_void);
                                }
                            }
                            fclose(f);
                        }
                        free(trieSerializedData as *mut libc::c_void);
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

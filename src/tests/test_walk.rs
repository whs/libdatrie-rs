use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _Trie;
    pub type _TrieState;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn trie_state_walkable_chars(
        s: *const TrieState,
        chars: *mut AlphaChar,
        chars_nelm: libc::c_int,
    ) -> libc::c_int;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn trie_root(trie: *const Trie) -> *mut TrieState;
    fn trie_state_clone(s: *const TrieState) -> *mut TrieState;
    fn trie_state_copy(dst: *mut TrieState, src: *const TrieState);
    fn trie_state_free(s: *mut TrieState);
    fn trie_state_walk(s: *mut TrieState, c: AlphaChar) -> Bool;
    fn trie_state_is_walkable(s: *const TrieState, c: AlphaChar) -> Bool;
    fn trie_free(trie: *mut Trie);
    fn trie_state_is_single(s: *const TrieState) -> Bool;
    fn trie_state_get_data(s: *const TrieState) -> TrieData;
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
pub type TrieState = _TrieState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DictRec {
    pub key: *mut AlphaChar,
    pub data: TrieData,
}
pub type DictRec = _DictRec;
pub type wchar_t = libc::c_int;
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
pub const FALSE: libc::c_int = DA_FALSE as libc::c_int;
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const TRIE_DATA_UNREAD: libc::c_int = 1 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut walk_dict: [DictRec; 7] = unsafe {
    [
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"p\0\0\0o\0\0\0o\0\0\0l\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 24], &[libc::c_int; 6]>(
                    b"p\0\0\0r\0\0\0i\0\0\0z\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"p\0\0\0r\0\0\0e\0\0\0v\0\0\0i\0\0\0e\0\0\0w\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"p\0\0\0r\0\0\0e\0\0\0p\0\0\0a\0\0\0r\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 32], &[libc::c_int; 8]>(
                    b"p\0\0\0r\0\0\0o\0\0\0d\0\0\0u\0\0\0c\0\0\0e\0\0\0\0\0\0\0",
                ))
                .as_ptr() as *mut AlphaChar,
                data: TRIE_DATA_UNREAD,
            };
            init
        },
        {
            let mut init = _DictRec {
                key: (*::core::mem::transmute::<&[u8; 36], &[libc::c_int; 9]>(
                    b"p\0\0\0r\0\0\0o\0\0\0g\0\0\0r\0\0\0e\0\0\0s\0\0\0s\0\0\0\0\0\0\0",
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
unsafe extern "C" fn is_walkables_include(
    mut c: AlphaChar,
    mut walkables: *const AlphaChar,
    mut n_elm: libc::c_int,
) -> Bool {
    while n_elm > 0 as libc::c_int {
        n_elm -= 1;
        if *walkables.offset(n_elm as isize) == c {
            return TRUE as Bool;
        }
    }
    return FALSE as Bool;
}
unsafe extern "C" fn print_walkables(mut walkables: *const AlphaChar, mut n_elm: libc::c_int) {
    let mut i: libc::c_int = 0;
    printf(b"{\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n_elm {
        if i > 0 as libc::c_int {
            printf(b", \0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"'%lc'\0" as *const u8 as *const libc::c_char,
            *walkables.offset(i as isize),
        );
        i += 1;
        i;
    }
    printf(b"}\0" as *const u8 as *const libc::c_char);
}
pub const ALPHABET_SIZE: libc::c_int = 256 as libc::c_int;
unsafe fn main_0() -> libc::c_int {
    let mut current_block: u64;
    let mut test_trie: *mut Trie = 0 as *mut Trie;
    let mut dict_p: *mut DictRec = 0 as *mut DictRec;
    let mut s: *mut TrieState = 0 as *mut TrieState;
    let mut t: *mut TrieState = 0 as *mut TrieState;
    let mut u: *mut TrieState = 0 as *mut TrieState;
    let mut walkables: [AlphaChar; 256] = [0; 256];
    let mut n: libc::c_int = 0;
    let mut is_failed: Bool = DA_FALSE;
    let mut data: TrieData = 0;
    msg_step(b"Preparing trie\0" as *const u8 as *const libc::c_char);
    test_trie = en_trie_new();
    if test_trie.is_null() {
        fprintf(
            stderr,
            b"Fail to create test trie\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        dict_p = walk_dict.as_mut_ptr();
        loop {
            if ((*dict_p).key).is_null() {
                current_block = 13109137661213826276;
                break;
            }
            if trie_store(test_trie, (*dict_p).key, (*dict_p).data) as u64 == 0 {
                printf(
                    b"Failed to add key '%ls', data %d.\n\0" as *const u8 as *const libc::c_char,
                    (*dict_p).key as *mut wchar_t,
                    (*dict_p).data,
                );
                current_block = 6506016528394336796;
                break;
            } else {
                dict_p = dict_p.offset(1);
                dict_p;
            }
        }
        match current_block {
            13109137661213826276 => {
                printf(
                    b"Now the trie structure is supposed to be:\n\n\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"          +---o-> (3) -o-> (4) -l-> [5]\n          |\n          |        +---i-> (7) -z-> (8) -e-> [9]\n          |        |\n(1) -p-> (2) -r-> (6) -e-> (10) -v-> (11) -i-> (12) -e-> (13) -w-> [14]\n                   |         |\n                   |         +---p-> (15) -a-> (16) -r-> (17) -e-> [18]\n                   |\n                   +---o-> (19) -d-> (20) -u-> (21) -c-> (22) -e-> [23]\n                             |\n                             +---g-> (24) -r-> (25) -e-> (26) -s-> (27) -s-> [28]\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                msg_step(b"Test walking\0" as *const u8 as *const libc::c_char);
                s = trie_root(test_trie);
                if s.is_null() {
                    printf(
                        b"Failed to get trie root state\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    msg_step(b"Test walking with 'p'\0" as *const u8 as *const libc::c_char);
                    if trie_state_is_walkable(s, 'p' as i32 as AlphaChar) as u64 == 0 {
                        printf(
                            b"Trie state is not walkable with 'p'\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if trie_state_walk(s, 'p' as i32 as AlphaChar) as u64 == 0 {
                        printf(b"Failed to walk with 'p'\n\0" as *const u8 as *const libc::c_char);
                    } else {
                        msg_step(
                            b"Now at (2), walkable chars should be {'o', 'r'}\0" as *const u8
                                as *const libc::c_char,
                        );
                        is_failed = FALSE as Bool;
                        n = trie_state_walkable_chars(s, walkables.as_mut_ptr(), ALPHABET_SIZE);
                        if 2 as libc::c_int != n {
                            printf(
                                b"Walkable chars should be exactly 2, got %d\n\0" as *const u8
                                    as *const libc::c_char,
                                n,
                            );
                            is_failed = TRUE as Bool;
                        }
                        if is_walkables_include('o' as i32 as AlphaChar, walkables.as_mut_ptr(), n)
                            as u64
                            == 0
                        {
                            printf(
                                b"Walkable chars do not include 'o'\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            is_failed = TRUE as Bool;
                        }
                        if is_walkables_include('r' as i32 as AlphaChar, walkables.as_mut_ptr(), n)
                            as u64
                            == 0
                        {
                            printf(
                                b"Walkable chars do not include 'r'\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            is_failed = TRUE as Bool;
                        }
                        if is_failed as u64 != 0 {
                            printf(b"Walkables = \0" as *const u8 as *const libc::c_char);
                            print_walkables(walkables.as_mut_ptr(), n);
                            printf(b"\n\0" as *const u8 as *const libc::c_char);
                        } else {
                            msg_step(
                                b"Try walking from (2) with 'o' to (3)\0" as *const u8
                                    as *const libc::c_char,
                            );
                            t = trie_state_clone(s);
                            if t.is_null() {
                                printf(
                                    b"Failed to clone trie state\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                if trie_state_walk(t, 'o' as i32 as AlphaChar) as u64 == 0 {
                                    printf(
                                        b"Failed to walk from (2) with 'o' to (3)\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block = 17005220323788385323;
                                } else if trie_state_is_single(t) as u64 == 0 {
                                    printf(
                                        b"(3) should be single, but isn't.\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block = 17005220323788385323;
                                } else {
                                    msg_step(
                                        b"Try walking from (3) with 'o' to (4)\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    if trie_state_walk(t, 'o' as i32 as AlphaChar) as u64 == 0 {
                                        printf(
                                            b"Failed to walk from (3) with 'o' to (4)\n\0"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 17005220323788385323;
                                    } else if trie_state_is_single(t) as u64 == 0 {
                                        printf(
                                            b"(4) should be single, but isn't.\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 17005220323788385323;
                                    } else {
                                        msg_step(
                                            b"Try walking from (4) with 'l' to (5)\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        if trie_state_walk(t, 'l' as i32 as AlphaChar) as u64 == 0 {
                                            printf(
                                                b"Failed to walk from (4) with 'l' to (5)\n\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                            );
                                            current_block = 17005220323788385323;
                                        } else if trie_state_is_walkable(
                                            t,
                                            0 as libc::c_int as AlphaChar,
                                        ) as u64
                                            == 0
                                        {
                                            printf(
                                                b"(5) should be terminal, but isn't.\n\0"
                                                    as *const u8
                                                    as *const libc::c_char,
                                            );
                                            current_block = 17005220323788385323;
                                        } else {
                                            msg_step(
                                                b"Try getting data from (5)\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                            data = trie_state_get_data(t);
                                            if TRIE_DATA_ERROR == data {
                                                printf(
                                                    b"Failed to get data from (5)\n\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                current_block = 17005220323788385323;
                                            } else if TRIE_DATA_UNREAD != data {
                                                printf(
                                                    b"Mismatched data from (5), expected %d, got %d\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    TRIE_DATA_UNREAD,
                                                    data,
                                                );
                                                current_block = 17005220323788385323;
                                            } else {
                                                msg_step(
                                                    b"Try walking from (2) with 'r' to (6)\0"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                );
                                                if trie_state_walk(s, 'r' as i32 as AlphaChar)
                                                    as u64
                                                    == 0
                                                {
                                                    printf(
                                                        b"Failed to walk from (2) with 'r' to (6)\n\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    current_block = 17005220323788385323;
                                                } else {
                                                    msg_step(
                                                        b"Now at (6), walkable chars should be {'e', 'i', 'o'}\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                    is_failed = FALSE as Bool;
                                                    n = trie_state_walkable_chars(
                                                        s,
                                                        walkables.as_mut_ptr(),
                                                        ALPHABET_SIZE,
                                                    );
                                                    if 3 as libc::c_int != n {
                                                        printf(
                                                            b"Walkable chars should be exactly 3, got %d\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            n,
                                                        );
                                                        is_failed = TRUE as Bool;
                                                    }
                                                    if is_walkables_include(
                                                        'e' as i32 as AlphaChar,
                                                        walkables.as_mut_ptr(),
                                                        n,
                                                    )
                                                        as u64
                                                        == 0
                                                    {
                                                        printf(
                                                            b"Walkable chars do not include 'e'\n\0"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        is_failed = TRUE as Bool;
                                                    }
                                                    if is_walkables_include(
                                                        'i' as i32 as AlphaChar,
                                                        walkables.as_mut_ptr(),
                                                        n,
                                                    )
                                                        as u64
                                                        == 0
                                                    {
                                                        printf(
                                                            b"Walkable chars do not include 'i'\n\0"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        is_failed = TRUE as Bool;
                                                    }
                                                    if is_walkables_include(
                                                        'o' as i32 as AlphaChar,
                                                        walkables.as_mut_ptr(),
                                                        n,
                                                    )
                                                        as u64
                                                        == 0
                                                    {
                                                        printf(
                                                            b"Walkable chars do not include 'o'\n\0"
                                                                as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        is_failed = TRUE as Bool;
                                                    }
                                                    if is_failed as u64 != 0 {
                                                        printf(
                                                            b"Walkables = \0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        print_walkables(walkables.as_mut_ptr(), n);
                                                        printf(
                                                            b"\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        current_block = 17005220323788385323;
                                                    } else {
                                                        msg_step(
                                                            b"Try walking from (6) with 'i' to (7)\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        trie_state_copy(t, s);
                                                        if trie_state_walk(
                                                            t,
                                                            'i' as i32 as AlphaChar,
                                                        )
                                                            as u64
                                                            == 0
                                                        {
                                                            printf(
                                                                b"Failed to walk from (6) with 'i' to (7)\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                            current_block = 17005220323788385323;
                                                        } else {
                                                            msg_step(
                                                                b"Try walking from (7) with 'z' to (8)\0" as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                            if trie_state_walk(
                                                                t,
                                                                'z' as i32 as AlphaChar,
                                                            )
                                                                as u64
                                                                == 0
                                                            {
                                                                printf(
                                                                    b"Failed to walk from (7) with 'z' to (8)\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                );
                                                                current_block =
                                                                    17005220323788385323;
                                                            } else if trie_state_is_single(t) as u64
                                                                == 0
                                                            {
                                                                printf(
                                                                    b"(7) should be single, but isn't.\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                );
                                                                current_block =
                                                                    17005220323788385323;
                                                            } else {
                                                                msg_step(
                                                                    b"Try walking from (8) with 'e' to (9)\0" as *const u8
                                                                        as *const libc::c_char,
                                                                );
                                                                if trie_state_walk(
                                                                    t,
                                                                    'e' as i32 as AlphaChar,
                                                                )
                                                                    as u64
                                                                    == 0
                                                                {
                                                                    printf(
                                                                        b"Failed to walk from (8) with 'e' to (9)\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                    current_block =
                                                                        17005220323788385323;
                                                                } else if trie_state_is_walkable(
                                                                    t,
                                                                    0 as libc::c_int as AlphaChar,
                                                                )
                                                                    as u64
                                                                    == 0
                                                                {
                                                                    printf(
                                                                        b"(9) should be terminal, but isn't.\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                    current_block =
                                                                        17005220323788385323;
                                                                } else {
                                                                    msg_step(
                                                                        b"Try getting data from (9)\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                    data = trie_state_get_data(t);
                                                                    if TRIE_DATA_ERROR == data {
                                                                        printf(
                                                                            b"Failed to get data from (9)\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                        );
                                                                        current_block =
                                                                            17005220323788385323;
                                                                    } else if TRIE_DATA_UNREAD
                                                                        != data
                                                                    {
                                                                        printf(
                                                                            b"Mismatched data from (9), expected %d, got %d\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            TRIE_DATA_UNREAD,
                                                                            data,
                                                                        );
                                                                        current_block =
                                                                            17005220323788385323;
                                                                    } else {
                                                                        msg_step(
                                                                            b"Try walking from (6) with 'e' to (10)\0" as *const u8
                                                                                as *const libc::c_char,
                                                                        );
                                                                        u = trie_state_clone(s);
                                                                        if u.is_null() {
                                                                            printf(
                                                                                b"Failed to clone trie state\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                            );
                                                                            current_block = 17005220323788385323;
                                                                        } else {
                                                                            if trie_state_walk(
                                                                                u,
                                                                                'e' as i32
                                                                                    as AlphaChar,
                                                                            )
                                                                                as u64
                                                                                == 0
                                                                            {
                                                                                printf(
                                                                                    b"Failed to walk from (6) with 'e' to (10)\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                );
                                                                                current_block = 17732100664789069381;
                                                                            } else {
                                                                                msg_step(
                                                                                    b"Now at (10), walkable chars should be {'p', 'v'}\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                );
                                                                                is_failed =
                                                                                    FALSE as Bool;
                                                                                n = trie_state_walkable_chars(
                                                                                    u,
                                                                                    walkables.as_mut_ptr(),
                                                                                    ALPHABET_SIZE,
                                                                                );
                                                                                if 2 as libc::c_int
                                                                                    != n
                                                                                {
                                                                                    printf(
                                                                                        b"Walkable chars should be exactly 2, got %d\n\0"
                                                                                            as *const u8 as *const libc::c_char,
                                                                                        n,
                                                                                    );
                                                                                    is_failed = TRUE
                                                                                        as Bool;
                                                                                }
                                                                                if is_walkables_include(
                                                                                    'p' as i32 as AlphaChar,
                                                                                    walkables.as_mut_ptr(),
                                                                                    n,
                                                                                ) as u64 == 0
                                                                                {
                                                                                    printf(
                                                                                        b"Walkable chars do not include 'p'\n\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    );
                                                                                    is_failed = TRUE as Bool;
                                                                                }
                                                                                if is_walkables_include(
                                                                                    'v' as i32 as AlphaChar,
                                                                                    walkables.as_mut_ptr(),
                                                                                    n,
                                                                                ) as u64 == 0
                                                                                {
                                                                                    printf(
                                                                                        b"Walkable chars do not include 'v'\n\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    );
                                                                                    is_failed = TRUE as Bool;
                                                                                }
                                                                                if is_failed as u64
                                                                                    != 0
                                                                                {
                                                                                    printf(
                                                                                        b"Walkables = \0" as *const u8 as *const libc::c_char,
                                                                                    );
                                                                                    print_walkables(walkables.as_mut_ptr(), n);
                                                                                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                                                                                    current_block = 17732100664789069381;
                                                                                } else {
                                                                                    msg_step(
                                                                                        b"Try walking from (10) with 'v' to (11)\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    );
                                                                                    trie_state_copy(
                                                                                        t, u,
                                                                                    );
                                                                                    if trie_state_walk(t, 'v' as i32 as AlphaChar) as u64 == 0 {
                                                                                        printf(
                                                                                            b"Failed to walk from (10) with 'v' to (11)\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        current_block = 17732100664789069381;
                                                                                    } else if trie_state_is_single(t) as u64 == 0 {
                                                                                        printf(
                                                                                            b"(11) should be single, but isn't.\n\0" as *const u8
                                                                                                as *const libc::c_char,
                                                                                        );
                                                                                        current_block = 17732100664789069381;
                                                                                    } else {
                                                                                        msg_step(
                                                                                            b"Try walking from (11) with 'i' to (12)\0" as *const u8
                                                                                                as *const libc::c_char,
                                                                                        );
                                                                                        if trie_state_walk(t, 'i' as i32 as AlphaChar) as u64 == 0 {
                                                                                            printf(
                                                                                                b"Failed to walk from (11) with 'i' to (12)\n\0"
                                                                                                    as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                            current_block = 17732100664789069381;
                                                                                        } else {
                                                                                            msg_step(
                                                                                                b"Try walking from (12) with 'e' to (13)\0" as *const u8
                                                                                                    as *const libc::c_char,
                                                                                            );
                                                                                            if trie_state_walk(t, 'e' as i32 as AlphaChar) as u64 == 0 {
                                                                                                printf(
                                                                                                    b"Failed to walk from (12) with 'e' to (13)\n\0"
                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                );
                                                                                                current_block = 17732100664789069381;
                                                                                            } else {
                                                                                                msg_step(
                                                                                                    b"Try walking from (13) with 'w' to (14)\0" as *const u8
                                                                                                        as *const libc::c_char,
                                                                                                );
                                                                                                if trie_state_walk(t, 'w' as i32 as AlphaChar) as u64 == 0 {
                                                                                                    printf(
                                                                                                        b"Failed to walk from (13) with 'w' to (14)\n\0"
                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                    );
                                                                                                    current_block = 17732100664789069381;
                                                                                                } else if trie_state_is_walkable(
                                                                                                    t,
                                                                                                    0 as libc::c_int as AlphaChar,
                                                                                                ) as u64 == 0
                                                                                                {
                                                                                                    printf(
                                                                                                        b"(14) should be terminal, but isn't.\n\0" as *const u8
                                                                                                            as *const libc::c_char,
                                                                                                    );
                                                                                                    current_block = 17732100664789069381;
                                                                                                } else {
                                                                                                    msg_step(
                                                                                                        b"Try getting data from (14)\0" as *const u8
                                                                                                            as *const libc::c_char,
                                                                                                    );
                                                                                                    data = trie_state_get_data(t);
                                                                                                    if TRIE_DATA_ERROR == data {
                                                                                                        printf(
                                                                                                            b"Failed to get data from (14)\n\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                        );
                                                                                                        current_block = 17732100664789069381;
                                                                                                    } else if TRIE_DATA_UNREAD != data {
                                                                                                        printf(
                                                                                                            b"Mismatched data from (14), expected %d, got %d\n\0"
                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                            TRIE_DATA_UNREAD,
                                                                                                            data,
                                                                                                        );
                                                                                                        current_block = 17732100664789069381;
                                                                                                    } else {
                                                                                                        msg_step(
                                                                                                            b"Try walking from (10) with 'p' to (15)\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                        );
                                                                                                        trie_state_copy(t, u);
                                                                                                        if trie_state_walk(t, 'p' as i32 as AlphaChar) as u64 == 0 {
                                                                                                            printf(
                                                                                                                b"Failed to walk from (10) with 'p' to (15)\n\0"
                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                            );
                                                                                                            current_block = 17732100664789069381;
                                                                                                        } else if trie_state_is_single(t) as u64 == 0 {
                                                                                                            printf(
                                                                                                                b"(15) should be single, but isn't.\n\0" as *const u8
                                                                                                                    as *const libc::c_char,
                                                                                                            );
                                                                                                            current_block = 17732100664789069381;
                                                                                                        } else {
                                                                                                            msg_step(
                                                                                                                b"Try walking from (15) with 'a' to (16)\0" as *const u8
                                                                                                                    as *const libc::c_char,
                                                                                                            );
                                                                                                            if trie_state_walk(t, 'a' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                printf(
                                                                                                                    b"Failed to walk from (15) with 'a' to (16)\n\0"
                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                );
                                                                                                                current_block = 17732100664789069381;
                                                                                                            } else {
                                                                                                                msg_step(
                                                                                                                    b"Try walking from (16) with 'r' to (17)\0" as *const u8
                                                                                                                        as *const libc::c_char,
                                                                                                                );
                                                                                                                if trie_state_walk(t, 'r' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                    printf(
                                                                                                                        b"Failed to walk from (16) with 'r' to (17)\n\0"
                                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                                    );
                                                                                                                    current_block = 17732100664789069381;
                                                                                                                } else {
                                                                                                                    msg_step(
                                                                                                                        b"Try walking from (17) with 'e' to (18)\0" as *const u8
                                                                                                                            as *const libc::c_char,
                                                                                                                    );
                                                                                                                    if trie_state_walk(t, 'e' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                        printf(
                                                                                                                            b"Failed to walk from (17) with 'e' to (18)\n\0"
                                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                                        );
                                                                                                                        current_block = 17732100664789069381;
                                                                                                                    } else if trie_state_is_walkable(
                                                                                                                        t,
                                                                                                                        0 as libc::c_int as AlphaChar,
                                                                                                                    ) as u64 == 0
                                                                                                                    {
                                                                                                                        printf(
                                                                                                                            b"(18) should be terminal, but isn't.\n\0" as *const u8
                                                                                                                                as *const libc::c_char,
                                                                                                                        );
                                                                                                                        current_block = 17732100664789069381;
                                                                                                                    } else {
                                                                                                                        msg_step(
                                                                                                                            b"Try getting data from (18)\0" as *const u8
                                                                                                                                as *const libc::c_char,
                                                                                                                        );
                                                                                                                        data = trie_state_get_data(t);
                                                                                                                        if TRIE_DATA_ERROR == data {
                                                                                                                            printf(
                                                                                                                                b"Failed to get data from (18)\n\0" as *const u8
                                                                                                                                    as *const libc::c_char,
                                                                                                                            );
                                                                                                                            current_block = 17732100664789069381;
                                                                                                                        } else if TRIE_DATA_UNREAD != data {
                                                                                                                            printf(
                                                                                                                                b"Mismatched data from (18), expected %d, got %d\n\0"
                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                                TRIE_DATA_UNREAD,
                                                                                                                                data,
                                                                                                                            );
                                                                                                                            current_block = 17732100664789069381;
                                                                                                                        } else {
                                                                                                                            trie_state_free(u);
                                                                                                                            msg_step(
                                                                                                                                b"Try walking from (6) with 'o' to (19)\0" as *const u8
                                                                                                                                    as *const libc::c_char,
                                                                                                                            );
                                                                                                                            if trie_state_walk(s, 'o' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                printf(
                                                                                                                                    b"Failed to walk from (6) with 'o' to (19)\n\0" as *const u8
                                                                                                                                        as *const libc::c_char,
                                                                                                                                );
                                                                                                                                current_block = 17005220323788385323;
                                                                                                                            } else {
                                                                                                                                msg_step(
                                                                                                                                    b"Now at (19), walkable chars should be {'d', 'g'}\0"
                                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                                );
                                                                                                                                is_failed = FALSE as Bool;
                                                                                                                                n = trie_state_walkable_chars(
                                                                                                                                    s,
                                                                                                                                    walkables.as_mut_ptr(),
                                                                                                                                    ALPHABET_SIZE,
                                                                                                                                );
                                                                                                                                if 2 as libc::c_int != n {
                                                                                                                                    printf(
                                                                                                                                        b"Walkable chars should be exactly 2, got %d\n\0"
                                                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                                                        n,
                                                                                                                                    );
                                                                                                                                    is_failed = TRUE as Bool;
                                                                                                                                }
                                                                                                                                if is_walkables_include(
                                                                                                                                    'd' as i32 as AlphaChar,
                                                                                                                                    walkables.as_mut_ptr(),
                                                                                                                                    n,
                                                                                                                                ) as u64 == 0
                                                                                                                                {
                                                                                                                                    printf(
                                                                                                                                        b"Walkable chars do not include 'd'\n\0" as *const u8
                                                                                                                                            as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    is_failed = TRUE as Bool;
                                                                                                                                }
                                                                                                                                if is_walkables_include(
                                                                                                                                    'g' as i32 as AlphaChar,
                                                                                                                                    walkables.as_mut_ptr(),
                                                                                                                                    n,
                                                                                                                                ) as u64 == 0
                                                                                                                                {
                                                                                                                                    printf(
                                                                                                                                        b"Walkable chars do not include 'g'\n\0" as *const u8
                                                                                                                                            as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    is_failed = TRUE as Bool;
                                                                                                                                }
                                                                                                                                if is_failed as u64 != 0 {
                                                                                                                                    printf(
                                                                                                                                        b"Walkables = \0" as *const u8 as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    print_walkables(walkables.as_mut_ptr(), n);
                                                                                                                                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                                                                                                                                    current_block = 17005220323788385323;
                                                                                                                                } else {
                                                                                                                                    msg_step(
                                                                                                                                        b"Try walking from (19) with 'd' to (20)\0" as *const u8
                                                                                                                                            as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    trie_state_copy(t, s);
                                                                                                                                    if trie_state_walk(t, 'd' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                        printf(
                                                                                                                                            b"Failed to walk from (19) with 'd' to (20)\n\0"
                                                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                                                        );
                                                                                                                                        current_block = 17005220323788385323;
                                                                                                                                    } else if trie_state_is_single(t) as u64 == 0 {
                                                                                                                                        printf(
                                                                                                                                            b"(20) should be single, but isn't.\n\0" as *const u8
                                                                                                                                                as *const libc::c_char,
                                                                                                                                        );
                                                                                                                                        current_block = 17005220323788385323;
                                                                                                                                    } else {
                                                                                                                                        msg_step(
                                                                                                                                            b"Try walking from (20) with 'u' to (21)\0" as *const u8
                                                                                                                                                as *const libc::c_char,
                                                                                                                                        );
                                                                                                                                        if trie_state_walk(t, 'u' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                            printf(
                                                                                                                                                b"Failed to walk from (20) with 'u' to (21)\n\0"
                                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                                            );
                                                                                                                                            current_block = 17005220323788385323;
                                                                                                                                        } else {
                                                                                                                                            msg_step(
                                                                                                                                                b"Try walking from (21) with 'c' to (22)\0" as *const u8
                                                                                                                                                    as *const libc::c_char,
                                                                                                                                            );
                                                                                                                                            if trie_state_walk(t, 'c' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                printf(
                                                                                                                                                    b"Failed to walk from (21) with 'c' to (22)\n\0"
                                                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                                                );
                                                                                                                                                current_block = 17005220323788385323;
                                                                                                                                            } else {
                                                                                                                                                msg_step(
                                                                                                                                                    b"Try walking from (22) with 'e' to (23)\0" as *const u8
                                                                                                                                                        as *const libc::c_char,
                                                                                                                                                );
                                                                                                                                                if trie_state_walk(t, 'e' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                    printf(
                                                                                                                                                        b"Failed to walk from (22) with 'e' to (23)\n\0"
                                                                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                                                                    );
                                                                                                                                                    current_block = 17005220323788385323;
                                                                                                                                                } else if trie_state_is_walkable(
                                                                                                                                                    t,
                                                                                                                                                    0 as libc::c_int as AlphaChar,
                                                                                                                                                ) as u64 == 0
                                                                                                                                                {
                                                                                                                                                    printf(
                                                                                                                                                        b"(23) should be terminal, but isn't.\n\0" as *const u8
                                                                                                                                                            as *const libc::c_char,
                                                                                                                                                    );
                                                                                                                                                    current_block = 17005220323788385323;
                                                                                                                                                } else {
                                                                                                                                                    msg_step(
                                                                                                                                                        b"Try getting data from (23)\0" as *const u8
                                                                                                                                                            as *const libc::c_char,
                                                                                                                                                    );
                                                                                                                                                    data = trie_state_get_data(t);
                                                                                                                                                    if TRIE_DATA_ERROR == data {
                                                                                                                                                        printf(
                                                                                                                                                            b"Failed to get data from (23)\n\0" as *const u8
                                                                                                                                                                as *const libc::c_char,
                                                                                                                                                        );
                                                                                                                                                        current_block = 17005220323788385323;
                                                                                                                                                    } else if TRIE_DATA_UNREAD != data {
                                                                                                                                                        printf(
                                                                                                                                                            b"Mismatched data from (23), expected %d, got %d\n\0"
                                                                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                                                                            TRIE_DATA_UNREAD,
                                                                                                                                                            data,
                                                                                                                                                        );
                                                                                                                                                        current_block = 17005220323788385323;
                                                                                                                                                    } else {
                                                                                                                                                        trie_state_free(t);
                                                                                                                                                        msg_step(
                                                                                                                                                            b"Try walking from (19) with 'g' to (24)\0" as *const u8
                                                                                                                                                                as *const libc::c_char,
                                                                                                                                                        );
                                                                                                                                                        if trie_state_walk(s, 'g' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                            printf(
                                                                                                                                                                b"Failed to walk from (19) with 'g' to (24)\n\0"
                                                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                                                            );
                                                                                                                                                        } else if trie_state_is_single(s) as u64 == 0 {
                                                                                                                                                            printf(
                                                                                                                                                                b"(24) should be single, but isn't.\n\0" as *const u8
                                                                                                                                                                    as *const libc::c_char,
                                                                                                                                                            );
                                                                                                                                                        } else {
                                                                                                                                                            msg_step(
                                                                                                                                                                b"Try walking from (24) with 'r' to (25)\0" as *const u8
                                                                                                                                                                    as *const libc::c_char,
                                                                                                                                                            );
                                                                                                                                                            if trie_state_walk(s, 'r' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                                printf(
                                                                                                                                                                    b"Failed to walk from (24) with 'r' to (25)\n\0"
                                                                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                                                                );
                                                                                                                                                            } else {
                                                                                                                                                                msg_step(
                                                                                                                                                                    b"Try walking from (25) with 'e' to (26)\0" as *const u8
                                                                                                                                                                        as *const libc::c_char,
                                                                                                                                                                );
                                                                                                                                                                if trie_state_walk(s, 'e' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                                    printf(
                                                                                                                                                                        b"Failed to walk from (25) with 'e' to (26)\n\0"
                                                                                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                                                                                    );
                                                                                                                                                                } else {
                                                                                                                                                                    msg_step(
                                                                                                                                                                        b"Try walking from (26) with 's' to (27)\0" as *const u8
                                                                                                                                                                            as *const libc::c_char,
                                                                                                                                                                    );
                                                                                                                                                                    if trie_state_walk(s, 's' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                                        printf(
                                                                                                                                                                            b"Failed to walk from (26) with 's' to (27)\n\0"
                                                                                                                                                                                as *const u8 as *const libc::c_char,
                                                                                                                                                                        );
                                                                                                                                                                    } else {
                                                                                                                                                                        msg_step(
                                                                                                                                                                            b"Try walking from (27) with 's' to (28)\0" as *const u8
                                                                                                                                                                                as *const libc::c_char,
                                                                                                                                                                        );
                                                                                                                                                                        if trie_state_walk(s, 's' as i32 as AlphaChar) as u64 == 0 {
                                                                                                                                                                            printf(
                                                                                                                                                                                b"Failed to walk from (27) with 's' to (28)\n\0"
                                                                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                                                                            );
                                                                                                                                                                        } else if trie_state_is_walkable(
                                                                                                                                                                            s,
                                                                                                                                                                            0 as libc::c_int as AlphaChar,
                                                                                                                                                                        ) as u64 == 0
                                                                                                                                                                        {
                                                                                                                                                                            printf(
                                                                                                                                                                                b"(28) should be terminal, but isn't.\n\0" as *const u8
                                                                                                                                                                                    as *const libc::c_char,
                                                                                                                                                                            );
                                                                                                                                                                        } else {
                                                                                                                                                                            msg_step(
                                                                                                                                                                                b"Try getting data from (28)\0" as *const u8
                                                                                                                                                                                    as *const libc::c_char,
                                                                                                                                                                            );
                                                                                                                                                                            data = trie_state_get_data(s);
                                                                                                                                                                            if TRIE_DATA_ERROR == data {
                                                                                                                                                                                printf(
                                                                                                                                                                                    b"Failed to get data from (28)\n\0" as *const u8
                                                                                                                                                                                        as *const libc::c_char,
                                                                                                                                                                                );
                                                                                                                                                                            } else if TRIE_DATA_UNREAD != data {
                                                                                                                                                                                printf(
                                                                                                                                                                                    b"Mismatched data from (28), expected %d, got %d\n\0"
                                                                                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                                                                                    TRIE_DATA_UNREAD,
                                                                                                                                                                                    data,
                                                                                                                                                                                );
                                                                                                                                                                            } else {
                                                                                                                                                                                trie_state_free(s);
                                                                                                                                                                                trie_free(test_trie);
                                                                                                                                                                                return 0 as libc::c_int;
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        current_block = 14967379764335304219;
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                14967379764335304219 => {}
                                                                                17005220323788385323 => {}
                                                                                _ => {
                                                                                    trie_state_free(u);
                                                                                    current_block = 17005220323788385323;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                match current_block {
                                    14967379764335304219 => {}
                                    _ => {
                                        trie_state_free(t);
                                    }
                                }
                            }
                        }
                    }
                    trie_state_free(s);
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

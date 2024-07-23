use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _AlphaMap;
    pub type _Trie;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alpha_map_add_range(
        alpha_map: *mut AlphaMap,
        begin: AlphaChar,
        end: AlphaChar,
    ) -> libc::c_int;
    fn trie_new(alpha_map: *const AlphaMap) -> *mut Trie;
    fn trie_new_from_file(path: *const libc::c_char) -> *mut Trie;
    fn trie_free(trie: *mut Trie);
    fn trie_save(trie: *mut Trie, path: *const libc::c_char) -> libc::c_int;
    fn trie_is_dirty(trie: *const Trie) -> Bool;
    fn trie_retrieve(trie: *const Trie, key: *const AlphaChar, o_data: *mut TrieData) -> Bool;
    fn trie_store(trie: *mut Trie, key: *const AlphaChar, data: TrieData) -> Bool;
    fn trie_delete(trie: *mut Trie, key: *const AlphaChar) -> Bool;
    fn trie_enumerate(
        trie: *const Trie,
        enum_func: TrieEnumFunc,
        user_data: *mut libc::c_void,
    ) -> Bool;
    fn alpha_map_new() -> *mut AlphaMap;
    fn alpha_map_free(alpha_map: *mut AlphaMap);
    fn alpha_char_strlen(str: *const AlphaChar) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type nl_item = libc::c_int;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_0 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_0 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_0 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_0 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_0 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_0 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_0 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_0 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_0 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_0 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_0 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_0 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_0 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_0 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_0 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_0 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_0 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_0 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_0 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_0 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_0 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_0 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_0 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_0 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_0 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_0 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_0 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_0 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_0 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_0 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_0 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_0 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_0 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_0 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_0 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_0 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_0 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_0 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_0 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_0 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_0 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_0 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_0 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_0 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_0 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_0 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_0 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_0 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_0 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_0 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_0 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_0 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_0 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_0 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_0 = 327684;
pub const __NOSTR: C2RustUnnamed_0 = 327683;
pub const __YESSTR: C2RustUnnamed_0 = 327682;
pub const __NOEXPR: C2RustUnnamed_0 = 327681;
pub const __YESEXPR: C2RustUnnamed_0 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_0 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_0 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_0 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_0 = 65539;
pub const __GROUPING: C2RustUnnamed_0 = 65538;
pub const THOUSEP: C2RustUnnamed_0 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_0 = 65537;
pub const RADIXCHAR: C2RustUnnamed_0 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_0 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_0 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_0 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_0 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_0 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_0 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_0 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_0 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_0 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_0 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_0 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_0 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_0 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_0 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_0 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_0 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_0 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_0 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_0 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_0 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_0 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_0 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_0 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_0 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_0 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_0 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_0 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_0 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_0 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_0 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_0 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_0 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_0 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_0 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_0 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_0 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_0 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_0 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_0 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_0 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_0 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_0 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_0 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_0 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_0 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_0 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_0 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_0 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_0 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_0 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_0 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_0 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_0 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_0 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_0 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_0 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_0 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_0 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_0 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_0 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_0 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_0 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_0 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_0 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_0 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_0 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_0 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_0 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_0 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_0 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_0 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_0 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_0 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_0 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_0 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_0 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_0 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_0 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_0 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_0 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_0 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_0 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_0 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_0 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_0 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_0 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_0 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_0 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_0 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_0 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_0 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_0 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_0 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_0 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_0 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_0 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_0 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_0 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_0 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_0 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_0 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_0 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_0 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_0 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_0 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_0 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_0 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_0 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_0 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_0 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_0 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_0 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_0 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_0 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_0 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_0 = 15;
pub const CODESET: C2RustUnnamed_0 = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_0 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_0 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_0 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_0 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_0 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_0 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_0 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_0 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_0 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_0 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_0 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_0 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_0 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_0 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_0 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_0 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_0 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_0 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_0 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_0 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_0 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_0 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_0 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_0 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_0 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_0 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_0 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_0 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_0 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_0 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_0 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_0 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_0 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_0 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_0 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_0 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_0 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_0 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_0 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_0 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_0 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_0 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_0 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_0 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_0 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_0 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_0 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_0 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_0 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_0 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_0 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_0 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_0 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_0 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_0 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_0 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_0 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_0 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_0 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_0 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_0 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_0 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_0 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_0 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_0 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_0 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_0 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_0 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_0 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_0 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_0 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_0 = 131195;
pub const __ALTMON_12: C2RustUnnamed_0 = 131194;
pub const __ALTMON_11: C2RustUnnamed_0 = 131193;
pub const __ALTMON_10: C2RustUnnamed_0 = 131192;
pub const __ALTMON_9: C2RustUnnamed_0 = 131191;
pub const __ALTMON_8: C2RustUnnamed_0 = 131190;
pub const __ALTMON_7: C2RustUnnamed_0 = 131189;
pub const __ALTMON_6: C2RustUnnamed_0 = 131188;
pub const __ALTMON_5: C2RustUnnamed_0 = 131187;
pub const __ALTMON_4: C2RustUnnamed_0 = 131186;
pub const __ALTMON_3: C2RustUnnamed_0 = 131185;
pub const __ALTMON_2: C2RustUnnamed_0 = 131184;
pub const __ALTMON_1: C2RustUnnamed_0 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_0 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_0 = 131181;
pub const _DATE_FMT: C2RustUnnamed_0 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_0 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_0 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_0 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_0 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_0 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_0 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_0 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_0 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_0 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_0 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_0 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_0 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_0 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_0 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_0 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_0 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_0 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_0 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_0 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_0 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_0 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_0 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_0 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_0 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_0 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_0 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_0 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_0 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_0 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_0 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_0 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_0 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_0 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_0 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_0 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_0 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_0 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_0 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_0 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_0 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_0 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_0 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_0 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_0 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_0 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_0 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_0 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_0 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_0 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_0 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_0 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_0 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_0 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_0 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_0 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_0 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_0 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_0 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_0 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_0 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_0 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_0 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_0 = 131117;
pub const ERA: C2RustUnnamed_0 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_0 = 131115;
pub const T_FMT: C2RustUnnamed_0 = 131114;
pub const D_FMT: C2RustUnnamed_0 = 131113;
pub const D_T_FMT: C2RustUnnamed_0 = 131112;
pub const PM_STR: C2RustUnnamed_0 = 131111;
pub const AM_STR: C2RustUnnamed_0 = 131110;
pub const MON_12: C2RustUnnamed_0 = 131109;
pub const MON_11: C2RustUnnamed_0 = 131108;
pub const MON_10: C2RustUnnamed_0 = 131107;
pub const MON_9: C2RustUnnamed_0 = 131106;
pub const MON_8: C2RustUnnamed_0 = 131105;
pub const MON_7: C2RustUnnamed_0 = 131104;
pub const MON_6: C2RustUnnamed_0 = 131103;
pub const MON_5: C2RustUnnamed_0 = 131102;
pub const MON_4: C2RustUnnamed_0 = 131101;
pub const MON_3: C2RustUnnamed_0 = 131100;
pub const MON_2: C2RustUnnamed_0 = 131099;
pub const MON_1: C2RustUnnamed_0 = 131098;
pub const ABMON_12: C2RustUnnamed_0 = 131097;
pub const ABMON_11: C2RustUnnamed_0 = 131096;
pub const ABMON_10: C2RustUnnamed_0 = 131095;
pub const ABMON_9: C2RustUnnamed_0 = 131094;
pub const ABMON_8: C2RustUnnamed_0 = 131093;
pub const ABMON_7: C2RustUnnamed_0 = 131092;
pub const ABMON_6: C2RustUnnamed_0 = 131091;
pub const ABMON_5: C2RustUnnamed_0 = 131090;
pub const ABMON_4: C2RustUnnamed_0 = 131089;
pub const ABMON_3: C2RustUnnamed_0 = 131088;
pub const ABMON_2: C2RustUnnamed_0 = 131087;
pub const ABMON_1: C2RustUnnamed_0 = 131086;
pub const DAY_7: C2RustUnnamed_0 = 131085;
pub const DAY_6: C2RustUnnamed_0 = 131084;
pub const DAY_5: C2RustUnnamed_0 = 131083;
pub const DAY_4: C2RustUnnamed_0 = 131082;
pub const DAY_3: C2RustUnnamed_0 = 131081;
pub const DAY_2: C2RustUnnamed_0 = 131080;
pub const DAY_1: C2RustUnnamed_0 = 131079;
pub const ABDAY_7: C2RustUnnamed_0 = 131078;
pub const ABDAY_6: C2RustUnnamed_0 = 131077;
pub const ABDAY_5: C2RustUnnamed_0 = 131076;
pub const ABDAY_4: C2RustUnnamed_0 = 131075;
pub const ABDAY_3: C2RustUnnamed_0 = 131074;
pub const ABDAY_2: C2RustUnnamed_0 = 131073;
pub const ABDAY_1: C2RustUnnamed_0 = 131072;
pub type iconv_t = *mut libc::c_void;
pub type Bool = libc::c_uint;
pub const DA_TRUE: Bool = 1;
pub const DA_FALSE: Bool = 0;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub type AlphaChar = uint32;
pub type TrieData = int32;
pub type AlphaMap = _AlphaMap;
pub type Trie = _Trie;
pub type TrieEnumFunc =
    Option<unsafe extern "C" fn(*const AlphaChar, TrieData, *mut libc::c_void) -> Bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProgEnv {
    pub path: *const libc::c_char,
    pub trie_name: *const libc::c_char,
    pub to_alpha_conv: iconv_t,
    pub from_alpha_conv: iconv_t,
    pub trie: *mut Trie,
}
pub const VERSION: [libc::c_char; 19] =
    unsafe { *::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"0.2.13-25-ga3a5155\0") };
pub const EXIT_FAILURE: libc::c_int = 1 as libc::c_int;
pub const EXIT_SUCCESS: libc::c_int = 0 as libc::c_int;
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
pub const __LC_CTYPE: libc::c_int = 0 as libc::c_int;
pub const LC_CTYPE: libc::c_int = __LC_CTYPE;
pub const CODESET_0: libc::c_int = CODESET as libc::c_int;
pub const TRUE: libc::c_int = DA_TRUE as libc::c_int;
pub const TRIE_DATA_ERROR: libc::c_int = -(1 as libc::c_int);
pub const ALPHA_ENC: [libc::c_char; 8] =
    unsafe { *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"UCS-4LE\0") };
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut env: ProgEnv = ProgEnv {
        path: 0 as *const libc::c_char,
        trie_name: 0 as *const libc::c_char,
        to_alpha_conv: 0 as *mut libc::c_void,
        from_alpha_conv: 0 as *mut libc::c_void,
        trie: 0 as *mut Trie,
    };
    let mut ret: libc::c_int = 0;
    env.path = b".\0" as *const u8 as *const libc::c_char;
    init_conv(&mut env);
    i = decode_switch(argc, argv, &mut env);
    if i == argc {
        usage(*argv.offset(0 as libc::c_int as isize), EXIT_FAILURE);
    }
    let fresh0 = i;
    i = i + 1;
    env.trie_name = *argv.offset(fresh0 as isize);
    if prepare_trie(&mut env) != 0 as libc::c_int {
        exit(EXIT_FAILURE);
    }
    ret = decode_command(argc - i, argv.offset(i as isize), &mut env);
    if close_trie(&mut env) != 0 as libc::c_int {
        exit(EXIT_FAILURE);
    }
    close_conv(&mut env);
    return ret;
}
unsafe extern "C" fn init_conv(mut env: *mut ProgEnv) {
    let mut prev_locale: *const libc::c_char = 0 as *const libc::c_char;
    let mut locale_codeset: *const libc::c_char = 0 as *const libc::c_char;
    prev_locale = setlocale(LC_CTYPE, b"\0" as *const u8 as *const libc::c_char);
    locale_codeset = nl_langinfo(CODESET_0);
    setlocale(LC_CTYPE, prev_locale);
    (*env).to_alpha_conv = iconv_open(ALPHA_ENC.as_ptr(), locale_codeset);
    (*env).from_alpha_conv = iconv_open(locale_codeset, ALPHA_ENC.as_ptr());
}
unsafe extern "C" fn conv_to_alpha(
    mut env: *mut ProgEnv,
    mut in_0: *const libc::c_char,
    mut out: *mut AlphaChar,
    mut out_size: size_t,
) -> size_t {
    let mut in_p: *mut libc::c_char = in_0 as *mut libc::c_char;
    let mut out_p: *mut libc::c_char = out as *mut libc::c_char;
    let mut in_left: size_t = strlen(in_0);
    let mut out_left: size_t =
        out_size.wrapping_mul(::core::mem::size_of::<AlphaChar>() as libc::c_ulong);
    let mut res: size_t = 0;
    let mut byte_p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    if ::core::mem::size_of::<AlphaChar>() as libc::c_ulong == 4 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof (AlphaChar) == 4\0" as *const u8 as *const libc::c_char,
            b"../tools/trietool.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                b"size_t conv_to_alpha(ProgEnv *, const char *, AlphaChar *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_2898: {
        if ::core::mem::size_of::<AlphaChar>() as libc::c_ulong == 4 as libc::c_int as libc::c_ulong
        {
        } else {
            __assert_fail(
                b"sizeof (AlphaChar) == 4\0" as *const u8 as *const libc::c_char,
                b"../tools/trietool.c\0" as *const u8 as *const libc::c_char,
                121 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                    b"size_t conv_to_alpha(ProgEnv *, const char *, AlphaChar *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    res = iconv(
        (*env).to_alpha_conv,
        &mut in_p as *mut *mut libc::c_char,
        &mut in_left,
        &mut out_p,
        &mut out_left,
    );
    if res == -(1 as libc::c_int) as size_t {
        return res;
    }
    res = 0 as libc::c_int as size_t;
    byte_p = out as *const libc::c_uchar;
    while res < out_size
        && byte_p.offset(3 as libc::c_int as isize)
            < out_p as *mut libc::c_uchar as *const libc::c_uchar
    {
        let fresh1 = res;
        res = res.wrapping_add(1);
        *out.offset(fresh1 as isize) = (*byte_p.offset(0 as libc::c_int as isize) as libc::c_int
            | (*byte_p.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | (*byte_p.offset(2 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
            | (*byte_p.offset(3 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int)
            as AlphaChar;
        byte_p = byte_p.offset(4 as libc::c_int as isize);
    }
    if res < out_size {
        *out.offset(res as isize) = 0 as libc::c_int as AlphaChar;
    }
    return res;
}
unsafe extern "C" fn conv_from_alpha(
    mut env: *mut ProgEnv,
    mut in_0: *const AlphaChar,
    mut out: *mut libc::c_char,
    mut out_size: size_t,
) -> size_t {
    let mut in_left: size_t = (alpha_char_strlen(in_0) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<AlphaChar>() as libc::c_ulong);
    let mut res: size_t = 0;
    if ::core::mem::size_of::<AlphaChar>() as libc::c_ulong == 4 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof (AlphaChar) == 4\0" as *const u8 as *const libc::c_char,
            b"../tools/trietool.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                b"size_t conv_from_alpha(ProgEnv *, const AlphaChar *, char *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3099: {
        if ::core::mem::size_of::<AlphaChar>() as libc::c_ulong == 4 as libc::c_int as libc::c_ulong
        {
        } else {
            __assert_fail(
                b"sizeof (AlphaChar) == 4\0" as *const u8 as *const libc::c_char,
                b"../tools/trietool.c\0" as *const u8 as *const libc::c_char,
                154 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"size_t conv_from_alpha(ProgEnv *, const AlphaChar *, char *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    res = 0 as libc::c_int as size_t;
    while *in_0.offset(res as isize) != 0 {
        let mut b: [libc::c_uchar; 4] = [0; 4];
        b[0 as libc::c_int as usize] =
            (*in_0.offset(res as isize) & 0xff as libc::c_int as AlphaChar) as libc::c_uchar;
        b[1 as libc::c_int as usize] = (*in_0.offset(res as isize) >> 8 as libc::c_int
            & 0xff as libc::c_int as AlphaChar)
            as libc::c_uchar;
        b[2 as libc::c_int as usize] = (*in_0.offset(res as isize) >> 16 as libc::c_int
            & 0xff as libc::c_int as AlphaChar)
            as libc::c_uchar;
        b[3 as libc::c_int as usize] = (*in_0.offset(res as isize) >> 24 as libc::c_int
            & 0xff as libc::c_int as AlphaChar)
            as libc::c_uchar;
        memcpy(
            &*in_0.offset(res as isize) as *const AlphaChar as *mut libc::c_char
                as *mut libc::c_void,
            b.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        res = res.wrapping_add(1);
        res;
    }
    res = iconv(
        (*env).from_alpha_conv,
        &mut in_0 as *mut *const AlphaChar as *mut *mut libc::c_char,
        &mut in_left,
        &mut out,
        &mut out_size,
    );
    *out = 0 as libc::c_int as libc::c_char;
    return res;
}
unsafe extern "C" fn close_conv(mut env: *mut ProgEnv) {
    iconv_close((*env).to_alpha_conv);
    iconv_close((*env).from_alpha_conv);
}
unsafe extern "C" fn full_path(
    mut path: *const libc::c_char,
    mut name: *const libc::c_char,
    mut ext: *const libc::c_char,
) -> *mut libc::c_char {
    let mut full_size: libc::c_int = (strlen(path))
        .wrapping_add(strlen(name))
        .wrapping_add(strlen(ext))
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut full_path_buff: *mut libc::c_char =
        malloc(full_size as libc::c_ulong) as *mut libc::c_char;
    sprintf(
        full_path_buff,
        b"%s/%s%s\0" as *const u8 as *const libc::c_char,
        path,
        name,
        ext,
    );
    return full_path_buff;
}
unsafe extern "C" fn prepare_trie(mut env: *mut ProgEnv) -> libc::c_int {
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = full_path(
        (*env).path,
        (*env).trie_name,
        b".tri\0" as *const u8 as *const libc::c_char,
    );
    (*env).trie = trie_new_from_file(path_name);
    free(path_name as *mut libc::c_void);
    if ((*env).trie).is_null() {
        let mut sbm: *mut FILE = 0 as *mut FILE;
        let mut alpha_map: *mut AlphaMap = 0 as *mut AlphaMap;
        path_name = full_path(
            (*env).path,
            (*env).trie_name,
            b".abm\0" as *const u8 as *const libc::c_char,
        );
        sbm = fopen(path_name, b"r\0" as *const u8 as *const libc::c_char);
        if sbm.is_null() {
            fprintf(
                stderr,
                b"Cannot open alphabet map file %s\n\0" as *const u8 as *const libc::c_char,
                path_name,
            );
            free(path_name as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        free(path_name as *mut libc::c_void);
        alpha_map = alpha_map_new();
        while !(fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            sbm,
        ))
        .is_null()
        {
            let mut b: libc::c_uint = 0;
            let mut e: libc::c_uint = 0;
            if sscanf(
                buff.as_mut_ptr(),
                b" [ %x , %x ] \0" as *const u8 as *const libc::c_char,
                &mut b as *mut libc::c_uint,
                &mut e as *mut libc::c_uint,
            ) != 2 as libc::c_int
            {
                continue;
            }
            if b > e {
                fprintf(
                    stderr,
                    b"Range begin (%x) > range end (%x)\n\0" as *const u8 as *const libc::c_char,
                    b,
                    e,
                );
            } else {
                alpha_map_add_range(alpha_map, b, e);
            }
        }
        (*env).trie = trie_new(alpha_map);
        alpha_map_free(alpha_map);
        fclose(sbm);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn close_trie(mut env: *mut ProgEnv) -> libc::c_int {
    if trie_is_dirty((*env).trie) as u64 != 0 {
        let mut path: *mut libc::c_char = full_path(
            (*env).path,
            (*env).trie_name,
            b".tri\0" as *const u8 as *const libc::c_char,
        );
        if trie_save((*env).trie, path) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Cannot save trie to %s\n\0" as *const u8 as *const libc::c_char,
                path,
            );
            free(path as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        free(path as *mut libc::c_void);
    }
    trie_free((*env).trie);
    return 0 as libc::c_int;
}
unsafe extern "C" fn decode_switch(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut opt_idx: libc::c_int = 0;
    opt_idx = 1 as libc::c_int;
    while opt_idx < argc && **argv.offset(opt_idx as isize) as libc::c_int == '-' as i32 {
        if strcmp(
            *argv.offset(opt_idx as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(opt_idx as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            usage(*argv.offset(0 as libc::c_int as isize), EXIT_FAILURE);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"-V\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(opt_idx as isize),
                b"--version\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                VERSION.as_ptr(),
            );
            exit(EXIT_FAILURE);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"-p\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(opt_idx as isize),
                b"--path\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            opt_idx += 1;
            (*env).path = *argv.offset(opt_idx as isize);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            break;
        } else {
            fprintf(
                stderr,
                b"Unknown option: %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(opt_idx as isize),
            );
            exit(EXIT_FAILURE);
        }
        opt_idx += 1;
        opt_idx;
    }
    return opt_idx;
}
unsafe extern "C" fn decode_command(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut opt_idx: libc::c_int = 0;
    opt_idx = 0 as libc::c_int;
    while opt_idx < argc {
        if strcmp(
            *argv.offset(opt_idx as isize),
            b"add\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            opt_idx += command_add(argc - opt_idx, argv.offset(opt_idx as isize), env);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"add-list\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            opt_idx += command_add_list(argc - opt_idx, argv.offset(opt_idx as isize), env);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"delete\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            opt_idx += command_delete(argc - opt_idx, argv.offset(opt_idx as isize), env);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"delete-list\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            opt_idx += command_delete_list(argc - opt_idx, argv.offset(opt_idx as isize), env);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"query\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            opt_idx += command_query(argc - opt_idx, argv.offset(opt_idx as isize), env);
        } else if strcmp(
            *argv.offset(opt_idx as isize),
            b"list\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opt_idx += 1;
            opt_idx;
            opt_idx += command_list(argc - opt_idx, argv.offset(opt_idx as isize), env);
        } else {
            fprintf(
                stderr,
                b"Unknown command: %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(opt_idx as isize),
            );
            return EXIT_FAILURE;
        }
        opt_idx += 1;
        opt_idx;
    }
    return EXIT_SUCCESS;
}
unsafe extern "C" fn command_add(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut opt_idx: libc::c_int = 0;
    opt_idx = 0 as libc::c_int;
    while opt_idx < argc {
        let mut key: *const libc::c_char = 0 as *const libc::c_char;
        let mut key_alpha: [AlphaChar; 256] = [0; 256];
        let mut data: TrieData = 0;
        let fresh2 = opt_idx;
        opt_idx = opt_idx + 1;
        key = *argv.offset(fresh2 as isize);
        data = if opt_idx < argc {
            let fresh3 = opt_idx;
            opt_idx = opt_idx + 1;
            atoi(*argv.offset(fresh3 as isize))
        } else {
            TRIE_DATA_ERROR
        };
        conv_to_alpha(
            env,
            key,
            key_alpha.as_mut_ptr(),
            (::core::mem::size_of::<[AlphaChar; 256]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<AlphaChar>() as libc::c_ulong),
        );
        if trie_store((*env).trie, key_alpha.as_mut_ptr(), data) as u64 == 0 {
            fprintf(
                stderr,
                b"Failed to add entry '%s' with data %d\n\0" as *const u8 as *const libc::c_char,
                key,
                data,
            );
        }
    }
    return opt_idx;
}
unsafe extern "C" fn command_add_list(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut enc_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut input_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt_idx: libc::c_int = 0;
    let mut saved_conv: iconv_t = 0 as *mut libc::c_void;
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 256] = [0; 256];
    enc_name = 0 as *const libc::c_char;
    opt_idx = 0 as libc::c_int;
    saved_conv = (*env).to_alpha_conv;
    if strcmp(
        *argv.offset(0 as libc::c_int as isize),
        b"-e\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"--encoding\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        opt_idx += 1;
        if opt_idx >= argc {
            fprintf(
                stderr,
                b"add-list option \"%s\" requires encoding name\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            return opt_idx;
        }
        let fresh4 = opt_idx;
        opt_idx = opt_idx + 1;
        enc_name = *argv.offset(fresh4 as isize);
    }
    if opt_idx >= argc {
        fprintf(
            stderr,
            b"add-list requires input word list file name\n\0" as *const u8 as *const libc::c_char,
        );
        return opt_idx;
    }
    let fresh5 = opt_idx;
    opt_idx = opt_idx + 1;
    input_name = *argv.offset(fresh5 as isize);
    if !enc_name.is_null() {
        let mut conv: iconv_t = iconv_open(ALPHA_ENC.as_ptr(), enc_name);
        if -(1 as libc::c_int) as iconv_t == conv {
            fprintf(
                stderr,
                b"Conversion from \"%s\" to \"%s\" is not supported.\n\0" as *const u8
                    as *const libc::c_char,
                enc_name,
                ALPHA_ENC.as_ptr(),
            );
            return opt_idx;
        }
        (*env).to_alpha_conv = conv;
    }
    input = fopen(input_name, b"r\0" as *const u8 as *const libc::c_char);
    if input.is_null() {
        fprintf(
            stderr,
            b"add-list: Cannot open input file \"%s\"\n\0" as *const u8 as *const libc::c_char,
            input_name,
        );
    } else {
        while !(fgets(
            line.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            input,
        ))
        .is_null()
        {
            let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut key_alpha: [AlphaChar; 256] = [0; 256];
            let mut data_val: TrieData = 0;
            key = string_trim(line.as_mut_ptr());
            if '\0' as i32 != *key as libc::c_int {
                data = key;
                while *data as libc::c_int != 0
                    && (strchr(
                        b"\t,\0" as *const u8 as *const libc::c_char,
                        *data as libc::c_int,
                    ))
                    .is_null()
                {
                    data = data.offset(1);
                    data;
                }
                if '\0' as i32 != *data as libc::c_int {
                    let fresh6 = data;
                    data = data.offset(1);
                    *fresh6 = '\0' as i32 as libc::c_char;
                    while *(*__ctype_b_loc()).offset(*data as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        data = data.offset(1);
                        data;
                    }
                }
                data_val = if '\0' as i32 != *data as libc::c_int {
                    atoi(data)
                } else {
                    TRIE_DATA_ERROR
                };
                conv_to_alpha(
                    env,
                    key,
                    key_alpha.as_mut_ptr(),
                    (::core::mem::size_of::<[AlphaChar; 256]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<AlphaChar>() as libc::c_ulong),
                );
                if trie_store((*env).trie, key_alpha.as_mut_ptr(), data_val) as u64 == 0 {
                    fprintf(
                        stderr,
                        b"Failed to add key '%s' with data %d.\n\0" as *const u8
                            as *const libc::c_char,
                        key,
                        data_val,
                    );
                }
            }
        }
        fclose(input);
    }
    if !enc_name.is_null() {
        iconv_close((*env).to_alpha_conv);
        (*env).to_alpha_conv = saved_conv;
    }
    return opt_idx;
}
unsafe extern "C" fn command_delete(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut opt_idx: libc::c_int = 0;
    opt_idx = 0 as libc::c_int;
    while opt_idx < argc {
        let mut key_alpha: [AlphaChar; 256] = [0; 256];
        conv_to_alpha(
            env,
            *argv.offset(opt_idx as isize),
            key_alpha.as_mut_ptr(),
            (::core::mem::size_of::<[AlphaChar; 256]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<AlphaChar>() as libc::c_ulong),
        );
        if trie_delete((*env).trie, key_alpha.as_mut_ptr()) as u64 == 0 {
            fprintf(
                stderr,
                b"No entry '%s'. Not deleted.\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(opt_idx as isize),
            );
        }
        opt_idx += 1;
        opt_idx;
    }
    return opt_idx;
}
unsafe extern "C" fn command_delete_list(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut enc_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut input_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt_idx: libc::c_int = 0;
    let mut saved_conv: iconv_t = 0 as *mut libc::c_void;
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 256] = [0; 256];
    enc_name = 0 as *const libc::c_char;
    opt_idx = 0 as libc::c_int;
    saved_conv = (*env).to_alpha_conv;
    if strcmp(
        *argv.offset(0 as libc::c_int as isize),
        b"-e\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"--encoding\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        opt_idx += 1;
        if opt_idx >= argc {
            fprintf(
                stderr,
                b"delete-list option \"%s\" requires encoding name\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            return opt_idx;
        }
        let fresh7 = opt_idx;
        opt_idx = opt_idx + 1;
        enc_name = *argv.offset(fresh7 as isize);
    }
    if opt_idx >= argc {
        fprintf(
            stderr,
            b"delete-list requires input word list file name\n\0" as *const u8
                as *const libc::c_char,
        );
        return opt_idx;
    }
    let fresh8 = opt_idx;
    opt_idx = opt_idx + 1;
    input_name = *argv.offset(fresh8 as isize);
    if !enc_name.is_null() {
        let mut conv: iconv_t = iconv_open(ALPHA_ENC.as_ptr(), enc_name);
        if -(1 as libc::c_int) as iconv_t == conv {
            fprintf(
                stderr,
                b"Conversion from \"%s\" to \"%s\" is not supported.\n\0" as *const u8
                    as *const libc::c_char,
                enc_name,
                ALPHA_ENC.as_ptr(),
            );
            return opt_idx;
        }
        (*env).to_alpha_conv = conv;
    }
    input = fopen(input_name, b"r\0" as *const u8 as *const libc::c_char);
    if input.is_null() {
        fprintf(
            stderr,
            b"delete-list: Cannot open input file \"%s\"\n\0" as *const u8 as *const libc::c_char,
            input_name,
        );
    } else {
        while !(fgets(
            line.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            input,
        ))
        .is_null()
        {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            p = string_trim(line.as_mut_ptr());
            if '\0' as i32 != *p as libc::c_int {
                let mut key_alpha: [AlphaChar; 256] = [0; 256];
                conv_to_alpha(
                    env,
                    p,
                    key_alpha.as_mut_ptr(),
                    (::core::mem::size_of::<[AlphaChar; 256]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<AlphaChar>() as libc::c_ulong),
                );
                if trie_delete((*env).trie, key_alpha.as_mut_ptr()) as u64 == 0 {
                    fprintf(
                        stderr,
                        b"No entry '%s'. Not deleted.\n\0" as *const u8 as *const libc::c_char,
                        p,
                    );
                }
            }
        }
        fclose(input);
    }
    if !enc_name.is_null() {
        iconv_close((*env).to_alpha_conv);
        (*env).to_alpha_conv = saved_conv;
    }
    return opt_idx;
}
unsafe extern "C" fn command_query(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    let mut key_alpha: [AlphaChar; 256] = [0; 256];
    let mut data: TrieData = 0;
    if argc == 0 as libc::c_int {
        fprintf(
            stderr,
            b"query: No key specified.\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    conv_to_alpha(
        env,
        *argv.offset(0 as libc::c_int as isize),
        key_alpha.as_mut_ptr(),
        (::core::mem::size_of::<[AlphaChar; 256]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<AlphaChar>() as libc::c_ulong),
    );
    if trie_retrieve((*env).trie, key_alpha.as_mut_ptr(), &mut data) as u64 != 0 {
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, data);
    } else {
        fprintf(
            stderr,
            b"query: Key '%s' not found.\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn list_enum_func(
    mut key: *const AlphaChar,
    mut key_data: TrieData,
    mut user_data: *mut libc::c_void,
) -> Bool {
    let mut env: *mut ProgEnv = user_data as *mut ProgEnv;
    let mut key_locale: [libc::c_char; 1024] = [0; 1024];
    conv_from_alpha(
        env,
        key,
        key_locale.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    printf(
        b"%s\t%d\n\0" as *const u8 as *const libc::c_char,
        key_locale.as_mut_ptr(),
        key_data,
    );
    return TRUE as Bool;
}
unsafe extern "C" fn command_list(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut env: *mut ProgEnv,
) -> libc::c_int {
    trie_enumerate(
        (*env).trie,
        Some(
            list_enum_func
                as unsafe extern "C" fn(*const AlphaChar, TrieData, *mut libc::c_void) -> Bool,
        ),
        env as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage(mut prog_name: *const libc::c_char, mut exit_status: libc::c_int) {
    printf(
        b"%s - double-array trie manipulator\n\0" as *const u8 as *const libc::c_char,
        prog_name,
    );
    printf(
        b"Usage: %s [OPTION]... TRIE CMD ARG ...\n\0" as *const u8 as *const libc::c_char,
        prog_name,
    );
    printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  -p, --path DIR           set trie directory to DIR [default=.]\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -h, --help               display this help and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -V, --version            output version information and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Commands:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  add  WORD DATA ...\n      Add WORD with DATA to trie\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  add-list [OPTION] LISTFILE\n      Add words and data listed in LISTFILE to trie\n      Options:\n          -e, --encoding ENC    specify character encoding of LISTFILE\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  delete WORD ...\n      Delete WORD from trie\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"  delete-list [OPTION] LISTFILE\n      Delete words listed in LISTFILE from trie\n      Options:\n          -e, --encoding ENC    specify character encoding of LISTFILE\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  query WORD\n      Query WORD data from trie\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"  list\n      List all words in trie\n\0" as *const u8 as *const libc::c_char);
    exit(exit_status);
}
unsafe extern "C" fn string_trim(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    while *s as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        s = s.offset(1);
        s;
    }
    p = s
        .offset(strlen(s) as isize)
        .offset(-(1 as libc::c_int as isize));
    while *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        p = p.offset(-1);
        p;
    }
    p = p.offset(1);
    *p = '\0' as i32 as libc::c_char;
    return s;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}

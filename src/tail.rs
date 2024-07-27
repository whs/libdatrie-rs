use std::{io, ptr, slice};
use std::io::{Cursor, Write};
use std::ptr::NonNull;

use ::libc;
use byteorder::{BigEndian, WriteBytesExt};

use crate::fileutils::wrap_cfile_nonnull;
use crate::trie::{TRIE_DATA_ERROR, TRIE_INDEX_ERROR, TrieChar, TrieData};
use crate::trie_string::{trie_char_strdup, trie_char_strlen, trie_char_strsize, TRIE_CHAR_TERM};
use crate::types::*;

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn file_read_int32(file: *mut FILE, o_val: *mut int32) -> Bool;
    fn file_read_int16(file: *mut FILE, o_val: *mut int16) -> Bool;
    fn file_read_chars(file: *mut FILE, buff: *mut libc::c_char, len: libc::c_int) -> Bool;
}

pub type size_t = libc::c_ulong;
pub type FILE = libc::FILE;
pub type int16 = libc::c_short;
pub type uint32 = libc::c_uint;
pub type int32 = libc::c_int;
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;

#[repr(C)]
pub(crate) struct Tail {
    pub num_tails: TrieIndex,
    pub tails: *mut TailBlock, // This is Box<[TailBlock; num_tails]>
    pub first_free: TrieIndex,
}

pub(crate) const TAIL_SIGNATURE: u32 = 0xdffcdffc;
pub(crate) const TAIL_START_BLOCKNO: i32 = 1;

impl Tail {
    fn blocks(&self) -> &[TailBlock] {
        if self.tails.is_null() {
            return &[];
        }

        unsafe { slice::from_raw_parts(self.tails.cast_const(), self.num_tails as usize) }
    }

    fn blocks_mut(&mut self) -> &mut [TailBlock] {
        if self.tails.is_null() {
            return &mut [];
        }

        unsafe { slice::from_raw_parts_mut(self.tails, self.num_tails as usize) }
    }

    pub(crate) fn get_suffix(&self, index: usize) -> Option<*mut TrieChar> {
        let index = index - TAIL_START_BLOCKNO as usize;
        self.blocks().get(index).map(|v| v.suffix)
    }

    pub(crate) fn get_data(&self, index: usize) -> Option<TrieData> {
        let index = index - TAIL_START_BLOCKNO as usize;
        self.blocks().get(index).map(|v| v.data)
    }

    pub(crate) fn set_data(&mut self, index: usize, data: TrieData) -> Option<()> {
        let index = index - TAIL_START_BLOCKNO as usize;
        match self.blocks_mut().get_mut(index) {
            Some(block) => {
                block.data = data;
                Some(())
            }
            None => None,
        }
    }

    pub(crate) fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        writer.write_u32::<BigEndian>(TAIL_SIGNATURE)?;
        writer.write_i32::<BigEndian>(self.first_free)?;
        writer.write_i32::<BigEndian>(self.num_tails)?;

        for block in self.blocks() {
            writer.write_i32::<BigEndian>(block.next_free)?;
            writer.write_i32::<BigEndian>(block.data)?;

            match unsafe { block.suffix.as_ref() } {
                Some(suffix) => {
                    let length = trie_char_strlen(suffix);
                    writer.write_i16::<BigEndian>(length as i16)?;
                    writer.write(unsafe { slice::from_raw_parts(suffix, length) })?;
                }
                None => {
                    // write the length
                    writer.write_i16::<BigEndian>(0)?;
                }
            };
        }

        Ok(())
    }
}

impl Default for Tail {
    fn default() -> Self {
        Tail {
            num_tails: 0,
            tails: ptr::null_mut(),
            first_free: 0,
        }
    }
}

impl Drop for Tail {
    fn drop(&mut self) {
        unsafe {
            for block in self.blocks_mut() {
                if !block.suffix.is_null() {
                    free(block.suffix.cast());
                }
            }
            free(self.tails.cast());
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TailBlock {
    pub next_free: TrieIndex,
    pub data: TrieData,
    pub suffix: *mut TrieChar,
}

#[deprecated(note = "Use Tail::default()")]
#[no_mangle]
pub(crate) extern "C" fn tail_new() -> *mut Tail {
    Box::into_raw(Box::new(Tail::default()))
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tail_fread(mut file: *mut FILE) -> *mut Tail {
    let mut current_block: u64;
    let mut save_pos: libc::c_long = 0;
    let mut t: *mut Tail = 0 as *mut Tail;
    let mut i: TrieIndex = 0;
    let mut sig: uint32 = 0;
    save_pos = ftell(file);
    if !(file_read_int32(file, &mut sig as *mut uint32 as *mut int32) as u64 == 0
        || TAIL_SIGNATURE != sig)
    {
        t = malloc(::core::mem::size_of::<Tail>() as libc::c_ulong) as *mut Tail;
        if !t.is_null() {
            if !(file_read_int32(file, &mut (*t).first_free) as u64 == 0
                || file_read_int32(file, &mut (*t).num_tails) as u64 == 0)
            {
                if !((*t).num_tails as libc::c_ulong
                    > SIZE_MAX.wrapping_div(::core::mem::size_of::<TailBlock>() as libc::c_ulong))
                {
                    (*t).tails = malloc(
                        ((*t).num_tails as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<TailBlock>() as libc::c_ulong),
                    ) as *mut TailBlock;
                    if !((*t).tails).is_null() {
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < (*t).num_tails) {
                                current_block = 15904375183555213903;
                                break;
                            }
                            let mut length: int16 = 0;
                            if file_read_int32(
                                file,
                                &mut (*((*t).tails).offset(i as isize)).next_free,
                            ) as u64
                                == 0
                                || file_read_int32(
                                    file,
                                    &mut (*((*t).tails).offset(i as isize)).data,
                                ) as u64
                                    == 0
                                || file_read_int16(file, &mut length) as u64 == 0
                            {
                                current_block = 3120876903627340905;
                                break;
                            }
                            let ref mut fresh0 = (*((*t).tails).offset(i as isize)).suffix;
                            *fresh0 =
                                malloc((length as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                                    as *mut TrieChar;
                            if ((*((*t).tails).offset(i as isize)).suffix).is_null() {
                                current_block = 3120876903627340905;
                                break;
                            }
                            if length as libc::c_int > 0 as libc::c_int {
                                if file_read_chars(
                                    file,
                                    (*((*t).tails).offset(i as isize)).suffix as *mut libc::c_char,
                                    length as libc::c_int,
                                ) as u64
                                    == 0
                                {
                                    free(
                                        (*((*t).tails).offset(i as isize)).suffix
                                            as *mut libc::c_void,
                                    );
                                    current_block = 3120876903627340905;
                                    break;
                                }
                            }
                            *((*((*t).tails).offset(i as isize)).suffix).offset(length as isize) =
                                TRIE_CHAR_TERM as TrieChar;
                            i += 1;
                            i;
                        }
                        match current_block {
                            15904375183555213903 => return t,
                            _ => {
                                while i > 0 as libc::c_int {
                                    i -= 1;
                                    free(
                                        (*((*t).tails).offset(i as isize)).suffix
                                            as *mut libc::c_void,
                                    );
                                }
                                free((*t).tails as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
            free(t as *mut libc::c_void);
        }
    }
    fseek(file, save_pos, SEEK_SET);
    return NULL as *mut Tail;
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tail_free(mut t: NonNull<Tail>) {
    let tail = Box::from_raw(t.as_ptr());
    drop(tail);
}

#[deprecated(note = "Use t.serialize()")]
#[no_mangle]
pub(crate) extern "C" fn tail_fwrite(t: *const Tail, file: NonNull<libc::FILE>) -> i32 {
    let tail = unsafe { &*t };
    let mut file = wrap_cfile_nonnull(file);
    match tail.serialize(&mut file) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tail_get_serialized_size(mut t: *const Tail) -> size_t {
    let mut static_count: size_t = (::core::mem::size_of::<int32>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<TrieIndex>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<TrieIndex>() as libc::c_ulong);
    let mut dynamic_count: size_t = 0 as libc::c_uint as size_t;
    if (*t).num_tails > 0 as libc::c_int {
        let mut i: TrieIndex = 0 as libc::c_int;
        dynamic_count = (dynamic_count as libc::c_ulong).wrapping_add(
            (::core::mem::size_of::<TrieIndex>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<TrieData>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<int16>() as libc::c_ulong)
                .wrapping_mul((*t).num_tails as libc::c_ulong),
        ) as size_t as size_t;
        while i < (*t).num_tails {
            if !((*((*t).tails).offset(i as isize)).suffix).is_null() {
                dynamic_count = dynamic_count.wrapping_add(trie_char_strsize(
                    (*((*t).tails).offset(i as isize)).suffix,
                ) as u64);
            }
            i += 1;
            i;
        }
    }
    return static_count.wrapping_add(dynamic_count);
}

#[deprecated(note = "Use t.serialize()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_serialize(
    t: *const Tail,
    mut ptr: NonNull<NonNull<[u8]>>,
) -> i32 {
    // FIXME: [u8] type is not actually stable
    let mut cursor = Cursor::new(ptr.as_mut().as_mut());
    (*t).serialize(&mut cursor).unwrap();
    // Move ptr
    ptr.write(ptr.as_ref().byte_offset(cursor.position() as isize));

    0
}

#[deprecated(note = "Use t.get_suffix()")]
#[no_mangle]
pub(crate) extern "C" fn tail_get_suffix(
    mut t: *const Tail,
    mut index: TrieIndex,
) -> *const TrieChar {
    let tail = unsafe { &*t };
    match tail.get_suffix(index as usize) {
        Some(v) => v,
        None => ptr::null(),
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tail_set_suffix(
    mut t: *mut Tail,
    mut index: TrieIndex,
    mut suffix: *const TrieChar,
) -> Bool {
    index -= TAIL_START_BLOCKNO;
    if index < (*t).num_tails {
        let mut tmp: *mut TrieChar = NULL as *mut TrieChar;
        if !suffix.is_null() {
            tmp = trie_char_strdup(suffix);
            if tmp.is_null() {
                return FALSE as Bool;
            }
        }
        if !((*((*t).tails).offset(index as isize)).suffix).is_null() {
            free((*((*t).tails).offset(index as isize)).suffix as *mut libc::c_void);
        }
        let ref mut fresh1 = (*((*t).tails).offset(index as isize)).suffix;
        *fresh1 = tmp;
        return TRUE as Bool;
    }
    return FALSE as Bool;
}

#[no_mangle]
pub(crate) unsafe extern "C" fn tail_add_suffix(
    mut t: *mut Tail,
    mut suffix: *const TrieChar,
) -> TrieIndex {
    let mut new_block: TrieIndex = 0;
    new_block = tail_alloc_block(t);
    if 0 as libc::c_int == new_block {
        return TRIE_INDEX_ERROR;
    }
    tail_set_suffix(t, new_block, suffix);
    return new_block;
}

unsafe fn tail_alloc_block(mut t: *mut Tail) -> TrieIndex {
    let mut block: TrieIndex = 0;
    if 0 as libc::c_int != (*t).first_free {
        block = (*t).first_free;
        (*t).first_free = (*((*t).tails).offset(block as isize)).next_free;
    } else {
        let mut new_block: *mut libc::c_void = 0 as *mut libc::c_void;
        block = (*t).num_tails;
        new_block = realloc(
            (*t).tails as *mut libc::c_void,
            (((*t).num_tails + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TailBlock>() as libc::c_ulong),
        );
        if new_block.is_null() {
            return TRIE_INDEX_ERROR;
        }
        (*t).tails = new_block as *mut TailBlock;
        (*t).num_tails += 1;
        (*t).num_tails;
    }
    (*((*t).tails).offset(block as isize)).next_free = -(1 as libc::c_int);
    (*((*t).tails).offset(block as isize)).data = TRIE_DATA_ERROR;
    let ref mut fresh2 = (*((*t).tails).offset(block as isize)).suffix;
    *fresh2 = NULL as *mut TrieChar;
    return block + TAIL_START_BLOCKNO;
}

unsafe fn tail_free_block(mut t: *mut Tail, mut block: TrieIndex) {
    let mut i: TrieIndex = 0;
    let mut j: TrieIndex = 0;
    block -= TAIL_START_BLOCKNO;
    if block >= (*t).num_tails {
        return;
    }
    (*((*t).tails).offset(block as isize)).data = TRIE_DATA_ERROR;
    if !((*((*t).tails).offset(block as isize)).suffix).is_null() {
        free((*((*t).tails).offset(block as isize)).suffix as *mut libc::c_void);
        let ref mut fresh3 = (*((*t).tails).offset(block as isize)).suffix;
        *fresh3 = NULL as *mut TrieChar;
    }
    j = 0 as libc::c_int;
    i = (*t).first_free;
    while i != 0 as libc::c_int && i < block {
        j = i;
        i = (*((*t).tails).offset(i as isize)).next_free;
    }
    (*((*t).tails).offset(block as isize)).next_free = i;
    if 0 as libc::c_int != j {
        (*((*t).tails).offset(j as isize)).next_free = block;
    } else {
        (*t).first_free = block;
    };
}

#[deprecated = "Use t.get_data().unwrap_or(TRIE_DATA_ERROR)"]
#[no_mangle]
pub(crate) extern "C" fn tail_get_data(mut t: *const Tail, mut index: TrieIndex) -> TrieData {
    let tail = unsafe { &*t };
    match tail.get_data(index as usize) {
        Some(v) => v,
        None => TRIE_DATA_ERROR,
    }
}

#[deprecated = "Use t.set_data()"]
#[no_mangle]
pub(crate) extern "C" fn tail_set_data(
    mut t: NonNull<Tail>,
    index: TrieIndex,
    data: TrieData,
) -> Bool {
    let tail = unsafe { t.as_mut() };
    match tail.set_data(index as usize, data) {
        Some(_) => TRUE,
        None => FALSE,
    }
}

// Delete suffix entry from the tail data.
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_delete(mut t: NonNull<Tail>, index: TrieIndex) {
    tail_free_block(t.as_mut(), index);
}

// Walk in tail with a string
//
// Walk in the tail data `t` at entry `s`, from given character position
// `*suffix_idx`, using `len` characters of given string `str`. On return,
// `*suffix_idx` is updated to the position after the last successful walk,
// and the function returns the total number of character successfully walked.
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_walk_str(
    t: *const Tail,
    s: TrieIndex,
    suffix_idx: *mut libc::c_short,
    str: *const TrieChar,
    len: libc::c_int,
) -> libc::c_int {
    let tail = unsafe { &*t };
    let Some(suffix) = tail.get_suffix(s as usize) else {
        return FALSE as libc::c_int;
    };
    let mut i = 0;
    let mut j = *suffix_idx;
    while i < len {
        if *str.offset(i as isize) as libc::c_int != *suffix.offset(j as isize) as libc::c_int {
            break;
        }
        i += 1;
        if TRIE_CHAR_TERM as libc::c_int == *suffix.offset(j as isize) as libc::c_int {
            break;
        }
        j += 1;
    }
    *suffix_idx = j;
    i
}

// Walk in tail with a character
//
// Walk in the tail data `t` at entry `s`, from given character position
// `*suffix_idx`, using given character `c`. If the walk is successful,
// it returns `TRUE`, and `*suffix_idx` is updated to the next character.
// Otherwise, it returns `FALSE`, and `*suffix_idx` is left unchanged.
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_walk_char(
    t: *const Tail,
    s: TrieIndex,
    suffix_idx: *mut i16,
    c: TrieChar,
) -> Bool {
    let tail = unsafe { &*t };
    let Some(suffix) = tail.get_suffix(s as usize) else {
        return FALSE;
    };
    let suffix_char = *suffix.offset(*suffix_idx as isize);
    if suffix_char == c {
        if TRIE_CHAR_TERM != suffix_char {
            *suffix_idx += 1;
        }
        return TRUE;
    }
    FALSE
}

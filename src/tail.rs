use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ptr::NonNull;
use std::{io, ptr, slice};

use ::libc;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fileutils::wrap_cfile_nonnull;
use crate::trie::{TrieChar, TrieData, TRIE_DATA_ERROR};
use crate::trie_string::{trie_char_clone, TRIE_CHAR_TERM};
use crate::types::*;

#[derive(Default)]
pub(crate) struct Tail {
    tails: Vec<TailBlock>,
    first_free: TrieIndex,
}

pub(crate) const TAIL_SIGNATURE: u32 = 0xdffcdffc;
pub(crate) const TAIL_START_BLOCKNO: TrieIndex = 1;

impl Tail {
    #[deprecated(note = "Use self.tails")]
    fn blocks(&self) -> &[TailBlock] {
        &self.tails
    }

    #[deprecated(note = "Use self.tails")]
    fn blocks_mut(&mut self) -> &mut [TailBlock] {
        &mut self.tails
    }

    pub(crate) fn get_suffix(&self, index: usize) -> Option<&[TrieChar]> {
        let index = index - TAIL_START_BLOCKNO as usize;
        match self.tails.get(index).map(|v| &v.suffix) {
            Some(Some(ref v)) => Some(&v),
            _ => None,
        }
    }

    pub(crate) fn set_suffix(&mut self, index: TrieIndex, suffix: Option<Box<[TrieChar]>>) -> bool {
        let index = (index - TAIL_START_BLOCKNO) as usize;

        if index >= self.tails.len() {
            return false;
        }

        self.tails[index].suffix = suffix;
        true
    }

    pub(crate) fn add_suffix(&mut self, suffix: Option<Box<[TrieChar]>>) -> TrieIndex {
        let new_block = self.alloc_block();
        self.set_suffix(new_block, suffix);
        new_block
    }

    pub(crate) fn get_data(&self, index: usize) -> Option<TrieData> {
        let index = index - TAIL_START_BLOCKNO as usize;
        self.tails.get(index).map(|v| v.data).flatten()
    }

    pub(crate) fn set_data(&mut self, index: usize, data: Option<TrieData>) -> Option<()> {
        let index = index - TAIL_START_BLOCKNO as usize;
        // TRIE_DATA_ERROR in C is mapped to None
        debug_assert_ne!(data, Some(TRIE_DATA_ERROR));
        match self.tails.get_mut(index) {
            Some(block) => {
                block.data = data;
                Some(())
            }
            None => None,
        }
    }

    pub(crate) fn delete(&mut self, index: TrieIndex) {
        self.free_block(index);
    }

    /// Walk in tail with a string
    ///
    /// Walk in the tail data `t` at entry `s`, from given character position
    /// `*suffix_idx`, using `len` characters of given string `str`.
    ///
    /// Return position after the last successful walk and the
    /// total number of character successfully walked.
    #[must_use]
    pub(crate) fn walk_str(&self, s: TrieIndex, suffix_idx: i16, str: &[TrieChar]) -> (i16, i32) {
        let Some(suffix) = self.get_suffix(s as usize) else {
            return (suffix_idx, 0);
        };

        let mut i = 0;
        let mut j = suffix_idx as usize;
        while i < str.len() {
            if str[i] != suffix[j] {
                break;
            }
            i += 1;

            // stop and stay at null-terminator
            if suffix[j] == TRIE_CHAR_TERM {
                break;
            }
            j += 1;
        }

        (j as i16, i as i32)
    }

    /// Walk in tail with a character
    ///
    /// Walk in the tail data `t` at entry `s`, from given character position
    /// `*suffix_idx`, using given character `c`. If the walk is successful,
    /// it returns `Some(next_character_idx)`. Otherwise, it returns `None`
    #[must_use]
    pub(crate) fn walk_char(&self, s: TrieIndex, suffix_idx: i16, c: TrieChar) -> Option<i16> {
        let suffix = self.get_suffix(s as usize)?;
        let suffix_char = suffix[suffix_idx as usize];
        if suffix_char == c {
            if TRIE_CHAR_TERM != suffix_char {
                return Some(suffix_idx + 1);
            }
            return Some(suffix_idx);
        }
        None
    }

    fn alloc_block(&mut self) -> TrieIndex {
        let mut block_idx;
        if self.first_free != 0 {
            block_idx = self.first_free;
            self.first_free = self.tails[block_idx as usize].next_free;

            self.tails[block_idx as usize].reset();
        } else {
            block_idx = self.tails.len() as TrieIndex;
            self.tails.push(TailBlock::default());
        }

        block_idx + TAIL_START_BLOCKNO
    }

    fn free_block(&mut self, block: TrieIndex) {
        let block_idx = (block - TAIL_START_BLOCKNO) as usize;

        // find insertion point
        let mut j = 0;
        let mut i = self.first_free as usize;
        while i != 0 && i < block_idx {
            j = i;
            i = self.tails[i].next_free as usize;
        }

        let Some(block) = self.tails.get_mut(block_idx) else {
            return;
        };
        block.reset();
        block.next_free = i as TrieIndex;

        if j != 0 {
            self.tails[j].next_free = block_idx as TrieIndex;
        } else {
            self.first_free = block_idx as TrieIndex;
        }
    }

    pub(crate) fn read<T: Read>(reader: &mut T) -> io::Result<Self> {
        if reader.read_u32::<BigEndian>()? != TAIL_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid signature",
            ));
        }
        let mut tail = Self::default();
        tail.first_free = reader.read_i32::<BigEndian>()?;
        let num_tails = reader.read_i32::<BigEndian>()?;

        if num_tails > (usize::MAX / size_of::<TailBlock>()) as i32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "block count too large",
            ));
        }

        // TODO: Consider using MaybeUninit
        let mut blocks = vec![TailBlock::default(); num_tails as usize];

        for block in &mut blocks {
            block.next_free = reader.read_i32::<BigEndian>()?;
            block.data = match reader.read_i32::<BigEndian>()? {
                TRIE_DATA_ERROR => None,
                value => Some(value),
            };
            block.suffix = None;

            let length = reader.read_i16::<BigEndian>()?;
            if length > 0 {
                let mut suffix = vec![TRIE_CHAR_TERM; (length + 1) as usize];
                reader.read_exact(&mut suffix[..(length as usize)])?;
                suffix[length as usize] = TRIE_CHAR_TERM;

                block.suffix = Some(suffix.into_boxed_slice());
            }
        }

        tail.tails = blocks;

        Ok(tail)
    }

    pub(crate) fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        writer.write_u32::<BigEndian>(TAIL_SIGNATURE)?;
        writer.write_i32::<BigEndian>(self.first_free)?;
        writer.write_i32::<BigEndian>(self.tails.len() as i32)?;

        for block in &self.tails {
            writer.write_i32::<BigEndian>(block.next_free)?;
            match block.data {
                Some(v) => writer.write_i32::<BigEndian>(v)?,
                None => writer.write_i32::<BigEndian>(TRIE_DATA_ERROR)?,
            }

            match &block.suffix {
                None => {
                    // write the length
                    writer.write_i16::<BigEndian>(0)?;
                }
                Some(suffix) => {
                    let length = suffix.len() - 1;
                    writer.write_i16::<BigEndian>(length as i16)?;
                    writer.write(&suffix[..length])?;
                }
            };
        }

        Ok(())
    }

    pub(crate) fn serialized_size(&self) -> usize {
        // This could potentially just be size_of::<TailBlock> but
        // to ensure compatibility with original code
        // we explicitly type out each fields' expected types
        const size_of_block: usize = size_of::<TrieIndex>() // next_free
            + size_of::<TrieData>() // data
            + size_of::<i16>(); // length

        size_of::<i32>() // TAIL_SIGNATURE
            + size_of::<TrieIndex>() // first_free
            + size_of::<TrieIndex>() // num_tails
            + (size_of_block * self.tails.len() as usize)
            + self.tails.iter().map(|block| {
            block.suffix.as_ref().map(|suffix| suffix.len() - 1).unwrap_or(0)
        }).sum::<usize>()
    }
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct TailBlock {
    next_free: TrieIndex,
    data: Option<TrieData>,
    suffix: Option<Box<[TrieChar]>>,
}

impl TailBlock {
    fn reset(&mut self) {
        self.next_free = -1;
        self.data = None;
        self.suffix = None;
    }
}

impl Default for TailBlock {
    fn default() -> Self {
        TailBlock {
            next_free: -1,
            data: None,
            suffix: None,
        }
    }
}

#[deprecated(note = "Use Tail::default()")]
#[no_mangle]
pub(crate) extern "C" fn tail_new() -> *mut Tail {
    Box::into_raw(Box::new(Tail::default()))
}

#[deprecated(note = "Use Tail::read(). Careful about file position on failure!")]
#[no_mangle]
pub(crate) extern "C" fn tail_fread(mut file: NonNull<libc::FILE>) -> *mut Tail {
    let mut file = wrap_cfile_nonnull(file);
    let save_pos = file.seek(SeekFrom::Current(0)).unwrap();

    match Tail::read(&mut file) {
        Ok(tail) => Box::into_raw(Box::new(tail)),
        Err(_) => {
            // Return to save_pos if read fail
            let _ = file.seek(SeekFrom::Start(save_pos));
            return ptr::null_mut();
        }
    }
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

#[deprecated(note = "Use t.serialized_size()")]
#[no_mangle]
pub(crate) extern "C" fn tail_get_serialized_size(t: *const Tail) -> usize {
    let tail = unsafe { &*t };
    tail.serialized_size()
}

#[deprecated(note = "Use t.serialize()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_serialize(
    t: *const Tail,
    mut ptr: NonNull<NonNull<[u8]>>,
) -> i32 {
    // FIXME: [u8] type is not actually stable ABI
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
        Some(v) => v.as_ptr(),
        None => ptr::null(),
    }
}

#[deprecated(note = "Use tail.set_suffix()")]
#[no_mangle]
pub(crate) extern "C" fn tail_set_suffix(
    mut t: NonNull<Tail>,
    index: TrieIndex,
    suffix: *const TrieChar,
) -> Bool {
    let tail = unsafe { t.as_mut() };
    let suffix = unsafe { suffix.as_ref() }.map(|v| trie_char_clone(v));
    match tail.set_suffix(index, suffix) {
        true => TRUE,
        false => FALSE,
    }
}

#[deprecated(note = "Clone the input into Rust and use tail.add_suffix().")]
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_add_suffix(
    mut t: NonNull<Tail>,
    suffix: *const TrieChar,
) -> TrieIndex {
    let tail = t.as_mut();
    let suffix = unsafe { suffix.as_ref() }.map(|v| trie_char_clone(v));

    tail.add_suffix(suffix)
}

#[deprecated(note = "Use t.get_data().unwrap_or(TRIE_DATA_ERROR)")]
#[no_mangle]
pub(crate) extern "C" fn tail_get_data(mut t: *const Tail, mut index: TrieIndex) -> TrieData {
    let tail = unsafe { &*t };
    match tail.get_data(index as usize) {
        Some(v) => v,
        None => TRIE_DATA_ERROR,
    }
}

#[deprecated(note = "Use t.set_data()")]
#[no_mangle]
pub(crate) extern "C" fn tail_set_data(
    mut t: NonNull<Tail>,
    index: TrieIndex,
    data: TrieData,
) -> Bool {
    let tail = unsafe { t.as_mut() };
    let data = match data {
        TRIE_DATA_ERROR => None,
        v => Some(v),
    };
    match tail.set_data(index as usize, data) {
        Some(_) => TRUE,
        None => FALSE,
    }
}

/// Delete suffix entry from the tail data.
#[deprecated(note = "Use t.delete()")]
#[no_mangle]
pub(crate) extern "C" fn tail_delete(mut t: NonNull<Tail>, index: TrieIndex) {
    let tail = unsafe { t.as_mut() };
    tail.delete(index)
}

/// Walk in tail with a string
///
/// Walk in the tail data `t` at entry `s`, from given character position
/// `*suffix_idx`, using `len` characters of given string `str`. On return,
/// `*suffix_idx` is updated to the position after the last successful walk,
/// and the function returns the total number of character successfully walked.
#[deprecated(note = "Use (*suffix_idx, walked) = t.walk_str()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_walk_str(
    t: *const Tail,
    s: TrieIndex,
    suffix_idx: *mut i16,
    str: *const TrieChar,
    len: i32,
) -> i32 {
    let tail = unsafe { &*t };
    let str = slice::from_raw_parts(str, len as usize);
    let (idx, walked) = tail.walk_str(s, *suffix_idx, str);
    *suffix_idx = idx;
    walked
}

/// Walk in tail with a character
///
/// Walk in the tail data `t` at entry `s`, from given character position
/// `*suffix_idx`, using given character `c`. If the walk is successful,
/// it returns `TRUE`, and `*suffix_idx` is updated to the next character.
/// Otherwise, it returns `FALSE`, and `*suffix_idx` is left unchanged.
#[deprecated(note = "Use Some(*suffix_idx) = t.walk_char()")]
#[no_mangle]
pub(crate) unsafe extern "C" fn tail_walk_char(
    t: *const Tail,
    s: TrieIndex,
    suffix_idx: *mut i16,
    c: TrieChar,
) -> Bool {
    let tail = unsafe { &*t };

    match tail.walk_char(s, *suffix_idx, c) {
        Some(idx) => {
            *suffix_idx = idx;
            TRUE
        }
        None => FALSE,
    }
}

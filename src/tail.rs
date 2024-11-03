use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::io;
#[cfg(feature = "std")]
use std::io::{Read, Write};

#[cfg(feature = "std")]
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::types::*;

#[derive(Default, Debug, Clone)]
pub(crate) struct Tail<TrieData> {
    tails: Vec<TailBlock<TrieData>>,
    first_free: TrieIndex,
}

const TAIL_SIGNATURE: u32 = 0xdffcdffc;
const TAIL_START_BLOCKNO: TrieIndex = 1;

impl<TrieData: Default> Tail<TrieData> {
    pub(crate) fn get_suffix(&self, index: TrieIndex) -> Option<&[TrieChar]> {
        let index = index - TAIL_START_BLOCKNO;
        match self.tails.get(index as usize).map(|v| &v.suffix) {
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

    pub(crate) fn get_data(&self, index: TrieIndex) -> Option<&TrieData> {
        let index = index - TAIL_START_BLOCKNO;
        self.tails.get(index as usize).map(|v| &v.data)
    }

    pub(crate) fn set_data(&mut self, index: TrieIndex, data: TrieData) -> Option<()> {
        let index = index - TAIL_START_BLOCKNO;
        match self.tails.get_mut(index as usize) {
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
    #[allow(unused)]
    pub(crate) fn walk_str(&self, s: TrieIndex, suffix_idx: i16, str: &[TrieChar]) -> (i16, i32) {
        let Some(suffix) = self.get_suffix(s) else {
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
        let suffix = self.get_suffix(s)?;
        let suffix_char = suffix[suffix_idx as usize];
        if suffix_char == c {
            if TRIE_CHAR_TERM != suffix_char {
                return Some(suffix_idx + 1);
            }
            return Some(suffix_idx);
        }
        None
    }

    pub(crate) fn is_walkable_char(&self, s: TrieIndex, suffix_idx: i16, c: TrieChar) -> bool {
        self.get_suffix(s).unwrap()[suffix_idx as usize] == c
    }

    fn alloc_block(&mut self) -> TrieIndex {
        let block_idx;
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
}

#[cfg(feature = "std")]
impl<TrieData: TrieSerializable> Tail<TrieData> {
    pub(crate) fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        writer.write_u32::<BigEndian>(TAIL_SIGNATURE)?;
        writer.write_i32::<BigEndian>(self.first_free)?;
        writer.write_i32::<BigEndian>(self.tails.len() as i32)?;

        for block in &self.tails {
            writer.write_i32::<BigEndian>(block.next_free)?;
            block.data.serialize(writer)?;

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
        const SIZE_OF_BLOCK: usize = size_of::<TrieIndex>() // next_free
            + size_of::<i16>(); // length

        size_of::<i32>() // TAIL_SIGNATURE
            + size_of::<TrieIndex>() // first_free
            + size_of::<TrieIndex>() // num_tails
            + (SIZE_OF_BLOCK * self.tails.len() as usize)
            + self.tails.iter().map(|block| {
            block.suffix.as_ref().map(|suffix| suffix.len() - 1).unwrap_or(0) + block.data.serialized_size()
        }).sum::<usize>()
    }
}

#[cfg(feature = "std")]
impl<TrieData: TrieDeserializable + Default> Tail<TrieData> {
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

        if num_tails as isize > isize::MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "block count too large",
            ));
        }

        let mut blocks = Vec::with_capacity(num_tails as usize);

        for _ in 0..num_tails {
            let mut block = TailBlock {
                next_free: reader.read_i32::<BigEndian>()?,
                data: TrieData::deserialize(reader)?,
                suffix: None,
            };

            let length = reader.read_i16::<BigEndian>()?;
            if length > 0 {
                let mut suffix = vec![TRIE_CHAR_TERM; (length + 1) as usize];
                reader.read_exact(&mut suffix[..(length as usize)])?;
                suffix[length as usize] = TRIE_CHAR_TERM;

                block.suffix = Some(suffix.into_boxed_slice());
            } else {
                // In the C version the reader always create suffix
                block.suffix = Some(Box::new([TRIE_CHAR_TERM]));
            }

            blocks.push(block);
        }

        tail.tails = blocks;

        Ok(tail)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TailBlock<TrieData> {
    next_free: TrieIndex,
    data: TrieData,
    suffix: Option<Box<[TrieChar]>>,
}

impl<TrieData: Default> TailBlock<TrieData> {
    fn reset(&mut self) {
        self.next_free = -1;
        self.data = Default::default();
        self.suffix = None;
    }
}

impl<TrieData: Default> Default for TailBlock<TrieData> {
    fn default() -> Self {
        TailBlock {
            next_free: -1,
            data: Default::default(),
            suffix: None,
        }
    }
}

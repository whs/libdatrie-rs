use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::{iter, slice};
use std::io;
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
#[cfg(feature = "cffi")]
use null_terminated::Nul;

pub type TrieIndex = i32;
pub const TRIE_INDEX_MAX: TrieIndex = 0x7fffffff;
pub const TRIE_INDEX_ERROR: TrieIndex = 0;

pub type AlphaChar = u32;
pub const ALPHA_CHAR_ERROR: AlphaChar = AlphaChar::MAX;

pub trait AsAlphaChar {
    fn as_alphachar(&self) -> Vec<AlphaChar>;
}

impl AsAlphaChar for &str {
    fn as_alphachar(&self) -> Vec<AlphaChar> {
        self.chars()
            .map(|v| v as AlphaChar)
            .chain(iter::once(0))
            .collect()
    }
}

pub trait AlphaCharToString {
    fn ac_to_string(&self) -> Option<String>;
}

impl AlphaCharToString for &[AlphaChar] {
    fn ac_to_string(&self) -> Option<String> {
        self.iter()
            .map_while(|v| {
                // Strip trailing null byte
                if *v == 0 {
                    return None;
                }
                if *v == ALPHA_CHAR_ERROR {
                    return Some(None);
                }
                Some(char::from_u32(*v))
            })
            .collect()
    }
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn alpha_char_strlen(str: *const AlphaChar) -> i32 {
    unsafe { Nul::new_unchecked(str) }.len() as i32
}

#[cfg(feature = "cffi")]
/// Return an AlphaChar string as slice, including the null byte
pub(crate) fn alpha_char_as_slice(str: *const AlphaChar) -> &'static [AlphaChar] {
    let len = alpha_char_strlen(str) as usize + 1;
    unsafe { slice::from_raw_parts(str, len) }
}

#[cfg(feature = "cffi")]
#[no_mangle]
pub extern "C" fn alpha_char_strcmp(str1: *const AlphaChar, str2: *const AlphaChar) -> i32 {
    let str1 = unsafe { Nul::new_unchecked(str1) };
    let str2 = unsafe { Nul::new_unchecked(str2) };
    match str1.cmp(str2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub type TrieChar = u8;
pub const TRIE_CHAR_TERM: TrieChar = '\0' as TrieChar;
pub const TRIE_CHAR_MAX: TrieChar = TrieChar::MAX;

pub trait TrieSerializable {
    fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()>;

    fn serialized_size(&self) -> usize {
        let mut buf = Vec::new();
        self.serialize(&mut buf).unwrap();
        buf.len()
    }
}

pub trait TrieDeserializable {
    fn deserialize<T: Read>(reader: &mut T) -> io::Result<Self>
    where
        Self: Sized;
}

impl TrieSerializable for i32 {
    fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        writer.write_i32::<BigEndian>(*self)
    }

    fn serialized_size(&self) -> usize {
        size_of::<i32>()
    }
}

impl TrieDeserializable for i32 {
    fn deserialize<T: Read>(reader: &mut T) -> io::Result<Self>
    where
        Self: Sized,
    {
        reader.read_i32::<BigEndian>()
    }
}

impl<T> TrieSerializable for Option<T>
where
    T: TrieSerializable,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Some(v) => {
                writer.write_u8(1)?;
                v.serialize(writer)
            }
            None => writer.write_u8(0),
        }
    }

    fn serialized_size(&self) -> usize {
        match self {
            Some(v) => 1 + v.serialized_size(),
            None => 1,
        }
    }
}

impl<T> TrieDeserializable for Option<T>
where
    T: TrieDeserializable,
{
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
    {
        let exists = reader.read_u8()?;
        match exists {
            1 => Ok(Some(T::deserialize(reader)?)),
            0 => Ok(None),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid exists byte",
            )),
        }
    }
}

impl TrieSerializable for Vec<u8> {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.len() as u64)?;
        writer.write(self)?;
        Ok(())
    }

    fn serialized_size(&self) -> usize {
        size_of::<u64>() + self.len()
    }
}

impl TrieDeserializable for Vec<u8> {
    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
    {
        let len = reader.read_u64::<BigEndian>()? as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

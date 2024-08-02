use crate::types::{TrieDeserializable, TrieSerializable};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

/// cbindgen:ignore
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Bool(u32);

pub(crate) const DA_TRUE: Bool = Bool(1);
pub(crate) const DA_FALSE: Bool = Bool(0);
pub(crate) const FALSE: Bool = DA_FALSE;
pub(crate) const TRUE: Bool = DA_TRUE;

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        match value {
            true => TRUE,
            false => FALSE,
        }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self.0 == 1
    }
}

#[deprecated(note = "TRIE_DATA_ERROR is for FFI use")]
#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct TrieData(pub i32);
#[deprecated(note = "TRIE_DATA_ERROR is for FFI use")]
pub const TRIE_DATA_ERROR: TrieData = TrieData(-1);

impl TrieSerializable for Option<TrieData> {
    fn serialize<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        match self {
            Some((v)) => writer.write_i32::<BigEndian>(v.0),
            None => writer.write_i32::<BigEndian>(-1),
        }
    }
}

impl TrieDeserializable for Option<TrieData> {
    fn deserialize<T: Read>(reader: &mut T) -> io::Result<Self> {
        Ok(match reader.read_i32::<BigEndian>()? {
            -1 => None,
            value => Some(TrieData(value)),
        })
    }
}

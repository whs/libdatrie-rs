////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

use std::ffi::OsStr;
use std::fs::File;
/// datrie implement a read-only libdatrie compatible data structure
use std::io;
use std::io::{BufReader, Read};

use alphamap::AlphaMap;
use darray::DArray;
use tail::Tail;

mod alphamap;
mod alphamaploader;
mod darray;
mod darrayloader;
mod tail;
mod tailloader;
mod test;

/// AlphaChar is the alphabet character used in words of a target language
pub type AlphaChar = u32;
pub type TrieIndex = i32;
pub type TrieChar = u8;
pub type TrieData = i32;

#[derive(Default, Clone)]
pub struct Trie {
    alpha_map: AlphaMap,
    darray: DArray,
    tail: Tail,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: &OsStr) -> io::Result<Self> {
        let mut fp = File::open(path)?;
        let mut buf = BufReader::new(fp);
        Self::from_reader(&mut buf)
    }

    pub fn from_reader<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut out = Self::default();
        out.load(reader)?;

        Ok(out)
    }

    pub fn load<T: Read>(&mut self, reader: &mut T) -> io::Result<()> {
        self.alpha_map.load(reader)?;
        self.darray.load(reader)?;
        self.tail.load(reader)?;

        Ok(())
    }

    pub fn root(&self) -> TrieState {
        TrieState {
            index: self.darray.get_root(),
            suffix_idx: 0,
            is_suffix: false,
        }
    }

    pub fn iter(&self) -> TrieIter {
        TrieIter {
            trie: self,
            root: self.root(),
            state: None,
            key: Vec::with_capacity(20),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TrieState {
    /// index in double-array/tail structures
    index: TrieIndex,
    /// suffix character offset, if in suffix
    suffix_idx: i16,
    /// whether it is currently in suffix part
    is_suffix: bool,
}

impl TrieState {
    pub fn walk(&self, c: AlphaChar) -> bool {
        todo!()
    }

    pub fn is_walkable(&self, c: AlphaChar) -> bool {
        todo!()
    }

    pub fn walkable_chars(&self, chars: &[AlphaChar]) -> i32 {
        todo!()
    }

    pub fn is_single(&self) -> bool {
        self.is_suffix
    }

    // Get value from a terminal state of trie
    // Returns None if called on non-terminal state
    pub fn get_data(&self) -> Option<TrieData> {
        todo!()
    }
}

pub struct TrieIter<'a> {
    trie: &'a Trie,
    root: TrieState,
    state: Option<TrieState>,
    key: Vec<TrieChar>,
}

impl<'a> TrieIter<'a> {
    fn get_key(&self) -> Option<Vec<AlphaChar>> {
        let s = self.state.as_ref()?;

        // if s is in tail, root == s
        if s.is_suffix {
            let tail_str = &self.trie.tail.get_suffix(s.index)?[s.suffix_idx as usize..];

            return Some(
                self.trie
                    .alpha_map
                    .to_alphas(tail_str.iter().cloned())
                    .into_iter()
                    .filter_map(|v| v)
                    .collect(),
            );
        } else {
            let tail_idx = self.trie.darray.get_tail_index(s.index)?;
            let tail_str = self.trie.tail.get_suffix(tail_idx)?;
            let prefix = self
                .trie
                .alpha_map
                .to_alphas_without_invalids(self.key.iter().cloned());
            let suffix = self
                .trie
                .alpha_map
                .to_alphas_without_invalids(tail_str.iter().cloned());
            return Some(prefix.chain(suffix).collect());
        }
    }

    fn get_data(&self) -> Option<TrieData> {
        let s = self.state.as_ref()?;

        if !s.is_suffix {
            if !self.trie.darray.is_separate(s.index) {
                return None;
            }

            return self
                .trie
                .tail
                .get_data(self.trie.darray.get_tail_index(s.index)?);
        } else {
            return self.trie.tail.get_data(s.index);
        }
    }
}

impl<'a> Iterator for TrieIter<'a> {
    type Item = (Vec<AlphaChar>, Option<TrieData>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state.as_mut() {
            None => {
                let mut state = self.state.insert(self.root.clone());

                // for tail state, we are already at the only entry
                if self.root.is_suffix {
                    return Some((self.get_key().unwrap(), self.get_data()));
                }

                state.index = self
                    .trie
                    .darray
                    .first_separate(self.root.index, &mut self.key)?;

                Some((self.get_key().unwrap(), self.get_data()))
            }
            Some(s) => {
                if s.is_suffix {
                    return None;
                }
                let sep =
                    self.trie
                        .darray
                        .next_separate(self.root.index, s.index, &mut self.key)?;
                s.index = sep;

                Some((self.get_key().unwrap(), self.get_data()))
            }
        }
    }
}

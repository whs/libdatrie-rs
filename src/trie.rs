use crate::darray::DArray;
use crate::tail::Tail;
use crate::{AlphaChar, AlphaMap, TrieChar, TrieData, TrieIndex};
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

#[derive(Clone, Debug)]
pub struct Trie {
    alpha_map: AlphaMap,
    darray: DArray,
    tail: Tail,
    is_dirty: bool,
}

impl Trie {
    pub fn new(alpha_map: AlphaMap) -> Self {
        Self {
            alpha_map,
            darray: DArray::new(),
            tail: Tail::new(),
            is_dirty: true,
        }
    }

    /// Open the provided file and create a Trie
    pub fn from_file(path: &OsStr) -> io::Result<Self> {
        let fp = File::open(path)?;
        let mut buf = BufReader::new(fp);
        Self::from_reader(&mut buf)
    }

    /// Read the content of the reader into a new Trie
    pub fn from_reader<T: Read>(reader: &mut T) -> io::Result<Self> {
        let mut out = Self {
            alpha_map: AlphaMap::new(),
            darray: DArray::default(),
            tail: Tail::default(),
            is_dirty: false,
        };
        out.load(reader)?;

        Ok(out)
    }

    /// Replace the contents of current Trie from contents in the reader
    pub fn load<T: Read>(&mut self, reader: &mut T) -> io::Result<()> {
        self.alpha_map.load(reader)?;
        self.darray.load(reader)?;
        self.tail.load(reader)?;

        Ok(())
    }

    /// Get a root state of this trie
    pub fn root(&self) -> TrieState {
        TrieState {
            trie: self,
            index: self.darray.get_root(),
            suffix_idx: 0,
            is_suffix: false,
        }
    }

    /// Create an iterator of this trie
    pub fn iter(&self) -> TrieIter {
        TrieIter {
            trie: self,
            root: self.root(),
            state: None,
            key: Vec::with_capacity(20),
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn retrieve(&self, key: &[AlphaChar]) -> Option<TrieData> {
        let mut state = self.darray.get_root();
        let mut sep = 0;
        for p in 0..key.len() {
            if self.darray.is_separate(state) {
                break;
            }

            let tc = match self.alpha_map.char_to_trie(key[p]) {
                Some(v) => v,
                None => return None,
            };

            match self.darray.walk(state, tc as TrieChar) {
                Some(new_state) => state = new_state,
                None => return None,
            }

            sep = p;
        }

        // walk through tail
        state = match self.darray.get_tail_index(state) {
            Some(v) => v,
            None => return None,
        };
        let mut suffix_idx = 0;
        for p in sep..key.len() {
            let tc = match self.alpha_map.char_to_trie(key[p]) {
                Some(v) => v,
                None => return None,
            };
            match self.tail.walk_char(state, suffix_idx, tc as TrieChar) {
                Some(v) => suffix_idx = v,
                None => return None,
            }
        }

        self.tail.get_data(state)
    }

    pub fn store(&mut self, key: &[AlphaChar], data: TrieData) -> Result<(), StoreError> {
        self.store_conditionally(key, data, true)
    }

    pub fn store_if_absent(&mut self, key: &[AlphaChar], data: TrieData) -> Result<(), StoreError> {
        self.store_conditionally(key, data, false)
    }

    fn store_conditionally(
        &mut self,
        key: &[AlphaChar],
        data: TrieData,
        overwrite: bool,
    ) -> Result<(), StoreError> {
        // walk through branches
        let mut state = self.darray.get_root();
        let mut key_iter = key.iter().copied().enumerate();
        while let Some((p, ch)) = key_iter.next() {
            if self.darray.is_separate(state) {
                break;
            }

            let tc = match self.alpha_map.char_to_trie(ch) {
                Some(v) => v,
                None => return Err(StoreError::KeyOutOfRange),
            };

            match self.darray.walk(state, tc as TrieChar) {
                Some(new_state) => {
                    state = new_state;
                }
                None => {
                    let key_str = match self.alpha_map.to_trie_str(&key[p..]) {
                        Some(v) => v,
                        None => return Err(StoreError::KeyOutOfRange),
                    };
                    return self.branch_in_branch(state, &key_str, data);
                }
            }
        }

        // walk through tail
        let mut t = match self.darray.get_tail_index(state) {
            Some(v) => v,
            None => return Err(StoreError::NotExists), // I think the original code expect this to not fail?
        };
        let mut suffix_idx = 0;
        while let Some((p, ch)) = key_iter.next() {
            let tc = match self.alpha_map.char_to_trie(key[p]) {
                Some(v) => v,
                None => return Err(StoreError::KeyOutOfRange),
            };
            /*if (!tail_walk_char (trie->tail, t, &suffix_idx, (TrieChar) tc)) {
                TrieChar *tail_str;
                Bool      res;

                tail_str = alpha_map_char_to_trie_str (trie->alpha_map, sep);
                if (!tail_str)
                    return FALSE;
                res = trie_branch_in_tail (trie, s, tail_str, data);
                free (tail_str);

                return res;
            }*/

            todo!();
        }

        if !overwrite {
            return Err(StoreError::Duplicate);
        }

        match self.tail.set_data(t, data) {
            Ok(_) => {}
            Err(_) => return Err(todo!()),
        }
        self.is_dirty = true;

        Ok(())
    }

    fn branch_in_branch(
        &mut self,
        sep_node: TrieIndex,
        suffix: &[TrieChar],
        data: TrieData,
    ) -> Result<(), StoreError> {
        let new_da = self
            .darray
            .insert_branch(sep_node, suffix[0])
            .ok_or(StoreError::Overflow)?;

        let new_tail = self.tail.add_suffix(&suffix[1..suffix.len()]);
        self.tail.set_data(new_tail, data).unwrap();
        self.darray.set_tail_index(new_da, new_tail);

        self.is_dirty = true;
        Ok(())
    }
}

// TODO: Better error type
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum StoreError {
    NotExists,
    /// The requested key has character outside of Trie's AlphaMap range
    KeyOutOfRange,
    /// The key is a duplicate. Only returned from [Trie.store_if_absent]
    Duplicate,
    /// Trie is full
    Overflow,
}

#[derive(Clone, Debug)]
pub struct TrieState<'trie> {
    trie: &'trie Trie,
    /// index in double-array/tail structures
    index: TrieIndex,
    /// suffix character offset, if in suffix
    suffix_idx: i16,
    /// whether it is currently in suffix part
    is_suffix: bool,
}

impl<'a> TrieState<'a> {
    pub fn walk(&mut self, c: AlphaChar) -> bool {
        let tc = match self.trie.alpha_map.char_to_trie(c) {
            Some(v) => v,
            None => return false,
        };

        if self.is_suffix {
            todo!();
            // return self.trie.tail.walk_char(self.index, self.suffix_idx, tc);
        }

        let next_state = match self.trie.darray.walk(self.index, tc as TrieChar) {
            Some(v) => v,
            None => return false,
        };

        self.index = next_state;

        if self.trie.darray.is_separate(self.index) {
            self.index = self.trie.darray.get_tail_index(self.index).unwrap();
            self.suffix_idx = 0;
            self.is_suffix = true;
        }

        return true;
    }

    pub fn is_walkable(&self, c: AlphaChar) -> bool {
        let tc = match self.trie.alpha_map.char_to_trie(c) {
            Some(v) => v,
            None => return false,
        };

        if self.is_suffix {
            todo!();
            // return self
            //     .trie
            //     .tail
            //     .is_walkable_char(self.index, self.suffix_idx, tc);
        }

        self.trie.darray.is_walkable(self.index, tc as TrieChar).unwrap()
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
    root: TrieState<'a>,
    state: Option<TrieState<'a>>,
    key: Vec<TrieChar>,
}

impl<'a> TrieIter<'a> {
    pub(crate) fn get_key(&self) -> Option<Vec<AlphaChar>> {
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

    pub(crate) fn get_data(&self) -> Option<TrieData> {
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

#[cfg(test)]
mod test {
    use crate::{test_utils, AlphaChar, AlphaMap, Trie};

    #[test]
    fn test_null_trie() {
        // Ported from test_null_trie.c
        let trie = test_utils::trie_new();
        assert_eq!(trie.iter().count(), 0)
    }

    #[test]
    fn test_byte_alpha() {
        // Ported from test_byte_alpha.c
        let mut alphamap = AlphaMap::new();
        alphamap.add_range(0x00, 0xff);
        let mut trie = Trie::new(alphamap);
        let key = [0xff, 0xff];
        trie.store(&key, 100).unwrap();
        assert_eq!(trie.retrieve(&key), Some(100));
    }

    #[test]
    fn test_iter() {
        let mut trie = test_utils::trie_new();
        for item in test_utils::DICT_SRC {
            let b: Vec<AlphaChar> = item.bytes().map(|v| v as AlphaChar).collect();
            trie.store(&b, 1);
        }
        todo!();
    }
}

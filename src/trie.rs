use crate::darray::DArray;
use crate::tail::Tail;
use crate::{AlphaChar, AlphaMap, TrieChar, TrieData, TrieIndex, TRIE_CHAR_TERM};
use std::ffi::OsStr;
use std::fmt::Write;
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

    /// Store data into trie, overwrite data if key is already present
    pub fn store(&mut self, key: &[AlphaChar], data: TrieData) -> Result<(), StoreError> {
        self.store_conditionally(key, data, true)
    }

    /// Store the data into trie, returning Err(StoreError::Duplicate) if key is already present without storing data
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
        // Iterator doesn't work quite well, but this add bound check
        let mut key_idx = 0;
        while !self.darray.is_separate(state) {
            let ch = key[key_idx];
            let tc = self
                .alpha_map
                .char_to_trie(ch)
                .ok_or(StoreError::KeyOutOfRange)?;

            match self.darray.walk(state, tc as TrieChar) {
                Some(new_state) => {
                    state = new_state;
                }
                None => {
                    let key_str = self
                        .alpha_map
                        .to_trie_str(&key[key_idx..])
                        .ok_or(StoreError::KeyOutOfRange)?;
                    return self.branch_in_branch(state, &key_str, data);
                }
            }

            if ch == 0 {
                break;
            }

            key_idx += 1;
        }

        // walk through tail
        let sep = &key[key_idx..];
        let t = self
            .darray
            .get_tail_index(state)
            .ok_or(StoreError::InternalError)?;
        let mut suffix_idx = 0;
        loop {
            let ch = key[key_idx];
            let tc = self
                .alpha_map
                .char_to_trie(ch)
                .ok_or(StoreError::KeyOutOfRange)?;

            match self.tail.walk_char(t, suffix_idx, tc as TrieChar) {
                Some(suffix) => suffix_idx = suffix,
                None => {
                    let tail_str = match self.alpha_map.to_trie_str(sep) {
                        Some(v) => v,
                        None => return Err(StoreError::KeyOutOfRange),
                    };
                    return self.branch_in_tail(state, &tail_str, data);
                }
            }

            if ch == 0 {
                break;
            }

            key_idx += 1;
        }

        if !overwrite {
            return Err(StoreError::Duplicate);
        }

        self.tail
            .set_data(t, data)
            .map_err(|_| StoreError::InternalError)?;
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

        let mut rest_suffix = suffix;
        if suffix[0] != TRIE_CHAR_TERM {
            rest_suffix = &suffix[1..];
            // TODO: suffix from the caller must be advanced
        }

        let new_tail = self.tail.add_suffix(rest_suffix);
        self.tail
            .set_data(new_tail, data)
            .map_err(|_| StoreError::InternalError)?;
        self.darray.set_tail_index(new_da, new_tail);

        self.is_dirty = true;
        Ok(())
    }

    fn branch_in_tail(
        &mut self,
        sep_node: TrieIndex,
        suffix: &[TrieChar],
        data: TrieData,
    ) -> Result<(), StoreError> {
        let old_tail = self
            .darray
            .get_tail_index(sep_node)
            .ok_or(StoreError::InternalError)?;
        let old_suffix = self
            .tail
            .get_suffix(old_tail)
            .ok_or(StoreError::InternalError)?;

        // TODO: On fail from this point on, call da_prune_upto & trie_da_set_tail_index
        let mut p = 0;
        let mut s = sep_node;
        while old_suffix.get(p) == suffix.get(p) {
            // TODO: insert_branch error could actually be overflow error or key mapping error
            let t = self
                .darray
                .insert_branch(s, old_suffix[p])
                .ok_or(StoreError::InternalError)?;
            s = t;
            p += 1;
        }

        let old_da = self
            .darray
            .insert_branch(s, old_suffix[p])
            .ok_or(StoreError::InternalError)?;

        if old_suffix[p] != TRIE_CHAR_TERM {
            p += 1;
        }

        self.tail
            .set_suffix(old_tail, old_suffix[p..].to_vec().as_ref())
            .map_err(|_| StoreError::InternalError)?;
        self.darray.set_tail_index(old_da, old_tail);

        // insert the new branch at the new separate point
        self.branch_in_branch(s, &suffix[p..], data)
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
    /// This should not be returned
    InternalError,
}

#[derive(Clone, Debug)]
pub struct TrieState<'trie> {
    trie: &'trie Trie,
    /// index in double-array/tail structures
    index: TrieIndex,
    /// suffix character offset, if in suffix
    suffix_idx: usize,
    /// whether it is currently in suffix part
    is_suffix: bool,
}

impl<'a> TrieState<'a> {
    pub fn to_iterator(&self) -> TrieIter<'a> {
        TrieIter {
            trie: self.trie,
            root: self.trie.root(),
            state: None,
            key: Vec::with_capacity(20),
        }
    }

    pub fn walk(&mut self, c: AlphaChar) -> bool {
        let tc = match self.trie.alpha_map.char_to_trie(c) {
            Some(v) => v,
            None => return false,
        };

        if self.is_suffix {
            let out = self
                .trie
                .tail
                .walk_char(self.index, self.suffix_idx, tc as TrieChar);
            match out {
                Some(idx) => {
                    self.suffix_idx = idx;
                    return true;
                }
                None => return false,
            }
        }

        let ret = self.trie.darray.walk(self.index, tc as TrieChar);
        match ret {
            Some(index) => {
                self.index = index;
                if self.trie.darray.is_separate(index) {
                    self.index = self.trie.darray.get_tail_index(index).unwrap();
                    self.suffix_idx = 0;
                    self.is_suffix = true;
                }
                return true;
            }
            _ => return false,
        }
    }

    pub fn is_walkable(&self, c: AlphaChar) -> bool {
        let tc = match self.trie.alpha_map.char_to_trie(c) {
            Some(v) => v,
            None => return false,
        };

        if self.is_suffix {
            return self
                .trie
                .tail
                .is_walkable_char(self.index, self.suffix_idx, tc as TrieChar);
        }

        self.trie
            .darray
            .is_walkable(self.index, tc as TrieChar)
            .unwrap()
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
        match self.is_suffix {
            true => {
                if self
                    .trie
                    .tail
                    .is_walkable_char(self.index, self.suffix_idx, TRIE_CHAR_TERM)
                {
                    return self.trie.tail.get_data(self.index);
                }
            }
            false => {
                // walk a terminal char to get the data from tail
                if let Some(next_index) = self.trie.darray.walk(self.index, TRIE_CHAR_TERM) {
                    if self.trie.darray.is_separate(next_index) {
                        return self
                            .trie
                            .tail
                            .get_data(self.trie.darray.get_tail_index(next_index).unwrap());
                    }
                }
            }
        }

        None
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

pub(crate) struct TriePrinter<'a> {
    trie: &'a Trie,
    output: String,
}

impl<'a> TriePrinter<'a> {
    fn build(&mut self) {
        self.output.write_str("digraph Trie {\n").unwrap();

        let index = self.trie.darray.get_root();
        self.walk_node(index);

        self.output.write_str("}").unwrap()
    }

    fn walk_node(&mut self, node: TrieIndex) {
        let trie_ch = self.trie.darray.get_base(node).unwrap() as TrieChar;

        let ch = self.trie.alpha_map.to_char(trie_ch).unwrap_or(0);
        write!(self.output, "{} [label=\"{}\"]\n", node, ch).unwrap();

        let mut keybuf = Vec::new();
        let mut subnode_opt = self.trie.darray.first_separate(node, &mut keybuf);
        while let Some(subnode) = subnode_opt {
            write!(self.output, "{} -> {}\n", node, subnode).unwrap();
            self.walk_node(subnode);
            subnode_opt = self.trie.darray.next_separate(node, subnode, &mut keybuf);
        }
    }

    pub fn output(&self) -> &str {
        return &self.output
    }
}

#[cfg(test)]
mod test {
    use crate::{alphachars_to_string, test_utils, AlphaChar, AlphaMap, ToAlphaChars, Trie};
    use std::collections::HashMap;
    use crate::trie::TriePrinter;

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
        // Ported from test_iterator.c
        let mut trie = test_utils::trie_new();
        for item in test_utils::DICT_SRC {
            let b: Vec<AlphaChar> = item.to_alphachars().unwrap();
            trie.store(&b, 1).unwrap();
        }

        // let mut printer = TriePrinter{trie: &trie, output: String::new()};
        // printer.build();
        // println!("{}", printer.output);

        let mut dict_found: HashMap<String, bool> =
            HashMap::from_iter(test_utils::DICT_SRC.iter().map(|v| (v.to_string(), false)));

        for (key, key_data) in trie.iter() {
            let key_str = alphachars_to_string(&key).expect("Key cannot be parsed as String");
            let value = dict_found
                .get_mut(&key_str)
                .expect(&format!("Key '{}' missing", key_str));
            *value = true;
            assert_eq!(key_data, Some(1));
            println!("Found key {}", key_str);
        }

        for (k, v) in dict_found {
            assert_eq!(v, true, "Key '{}' not found", k);
        }
    }

    #[test]
    fn test_term_state() {
        // Ported from test_term_state.c
        let mut trie = test_utils::trie_new();
        trie.store("ab".to_alphachars().unwrap().as_ref(), 1)
            .unwrap();
        trie.store("abc".to_alphachars().unwrap().as_ref(), 2)
            .unwrap();

        let mut trie_state = trie.root();

        assert!(trie_state.walk('a'.into()));
        assert_eq!(trie_state.get_data(), None, "Incorrect data at 'a'");

        assert!(trie_state.walk('b'.into()));
        assert_eq!(trie_state.get_data(), Some(1), "Incorrect data at 'ab'");

        assert!(trie_state.walk('c'.into()));
        assert_eq!(trie_state.get_data(), Some(2), "Incorrect data at 'abc'");
    }
}

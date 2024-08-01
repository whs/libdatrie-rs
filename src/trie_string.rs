use std::ops::{Deref, DerefMut};

pub type TrieChar = u8;
pub const TRIE_CHAR_TERM: TrieChar = '\0' as TrieChar;
pub const TRIE_CHAR_MAX: TrieChar = TrieChar::MAX;

#[derive(Clone, Default, Debug)]
pub(crate) struct TrieString {
    inner: Vec<TrieChar>,
    str_len: usize,
}

impl TrieString {
    pub(crate) fn length(&self) -> usize {
        self.str_len
    }

    pub(crate) fn clear(&mut self) {
        self.str_len = 0;
        self.inner.clear();
    }

    pub(crate) fn append(&mut self, c: TrieChar) {
        self.inner.truncate(self.str_len);
        self.inner.push(c);
        self.str_len += 1;
    }

    pub(crate) fn pop(&mut self) -> Option<TrieChar> {
        let out = self.inner.pop();
        if out.is_some() {
            self.str_len -= 1;
        }

        out
    }
}

impl Deref for TrieString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner[..self.str_len]
    }
}

impl DerefMut for TrieString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner[..self.str_len]
    }
}

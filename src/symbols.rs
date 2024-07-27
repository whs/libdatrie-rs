use crate::trie_string::TrieChar;
use arrayvec::ArrayVec;
use std::slice::Iter;

#[derive(Debug, Default)]
pub(crate) struct Symbols {
    symbols: ArrayVec<TrieChar, { TrieChar::MAX as usize }>,
}

impl Symbols {
    pub(crate) fn add_fast(&mut self, ch: TrieChar) {
        self.symbols.push(ch);
    }

    pub(crate) fn add(&mut self, ch: TrieChar) {
        let insertion_point = self.symbols.partition_point(|no| *no < ch);

        if self.symbols.get(insertion_point).copied() == Some(ch) {
            return;
        }

        self.symbols.insert(insertion_point, ch);
    }

    pub(crate) fn num(&self) -> usize {
        self.symbols.len()
    }

    pub(crate) fn get(&self, index: usize) -> Option<TrieChar> {
        self.symbols.get(index).copied()
    }

    pub(crate) fn iter(&self) -> Iter<'_, TrieChar> {
        self.symbols.iter()
    }
}

#[deprecated]
#[no_mangle]
pub(crate) unsafe extern "C" fn symbols_free(syms: *mut Symbols) {
    drop(Box::from_raw(syms))
}
#[deprecated(note="Use syms.get().unwrap()")]
#[no_mangle]
pub(crate) extern "C" fn symbols_get(syms: *const Symbols, index: i32) -> TrieChar {
    unsafe { &*syms }.get(index as usize).unwrap()
}
#[deprecated(note="Use syms.num()")]
#[no_mangle]
pub(crate) extern "C" fn symbols_num(syms: *const Symbols) -> i32 {
    unsafe { &*syms }.num() as i32
}

#[cfg(test)]
mod tests {
    use crate::symbols::Symbols;
    use crate::trie_string::TrieChar;

    #[test]
    fn test_symbol() {
        let mut symbols = Symbols::default();
        for i in (0..=10).rev() {
            symbols.add(i)
        }
        assert_eq!(symbols.num(), 11);
        for i in 0..=10 {
            assert_eq!(symbols.get(i), Some(i as TrieChar));
        }
        assert_eq!(symbols.get(11), None);
    }

    #[test]
    fn test_symbol_dupe() {
        let mut symbols = Symbols::default();
        symbols.add(1);
        symbols.add(1);
        assert_eq!(symbols.num(), 1);
    }

    #[test]
    fn test_symbol_large() {
        let mut symbols = Symbols::default();
        for i in (0..TrieChar::MAX).rev() {
            symbols.add(i)
        }
        assert_eq!(symbols.num(), TrieChar::MAX as usize);
        for i in 0..TrieChar::MAX as usize {
            assert_eq!(symbols.get(i), Some(i as TrieChar));
        }
    }
}

use crate::types::TrieChar;
use alloc::slice::Iter;
use arrayvec::ArrayVec;

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

    #[allow(unused)]
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

#[cfg(test)]
mod tests {
    use crate::symbols::Symbols;
    use crate::types::TrieChar;

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

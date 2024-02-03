use crate::TrieChar;
use std::slice::Iter;

#[derive(Debug)]
pub(crate) struct Symbols {
    num: usize,
    symbols: [TrieChar; TrieChar::MAX as usize],
}

impl Symbols {
    pub(crate) fn new() -> Symbols {
        Symbols {
            num: 0,
            symbols: [0; TrieChar::MAX as usize],
        }
    }

    pub(crate) fn add_fast(&mut self, ch: TrieChar) {
        let index = self.num;
        self.num += 1;
        self.symbols[index] = ch;
    }

    pub(crate) fn add(&mut self, ch: TrieChar) {
        let insertion_point = self.symbols[0..self.num].partition_point(|no| *no < ch);

        if self.symbols[insertion_point] == ch {
            return;
        }

        let end = self.symbols.len() - 1;
        self.symbols
            .copy_within(insertion_point..end, insertion_point + 1);
        self.symbols[insertion_point] = ch;
        self.num += 1;
    }

    pub(crate) fn num(&self) -> usize {
        self.num
    }

    pub(crate) fn get(&self, index: usize) -> Option<TrieChar> {
        if index >= self.num {
            return None;
        }
        self.symbols.get(index).copied()
    }

    pub(crate) fn iter(&self) -> Iter<'_, TrieChar> {
        self.symbols[0..self.num].iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::Symbols;
    use crate::TrieChar;

    #[test]
    fn test_symbol() {
        let mut symbols = Symbols::new();
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
        let mut symbols = Symbols::new();
        symbols.add(1);
        symbols.add(1);
        assert_eq!(symbols.num(), 1);
    }

    #[test]
    fn test_symbol_large() {
        let mut symbols = Symbols::new();
        for i in (0..TrieChar::MAX).rev() {
            symbols.add(i)
        }
        assert_eq!(symbols.num(), TrieChar::MAX as usize);
        for i in 0..TrieChar::MAX as usize {
            assert_eq!(symbols.get(i), Some(i as TrieChar));
        }
    }
}

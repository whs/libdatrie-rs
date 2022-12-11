#[cfg(test)]
mod test {
    use crate::Trie;
    use std::path::PathBuf;

    #[test]
    fn load_libthai() {
        let path = PathBuf::from("/usr/share/libthai/thbrk.tri");

        if !path.exists() {
            println!("Skipping test as {:?} is missing", path);
            return
        }

        let ctrie = cdatrie::Trie::from_file(path.as_os_str()).unwrap();
        let item_count = ctrie.iter().count();
        assert!(item_count > 0);

        let trie = Trie::from_file(path.as_os_str()).unwrap();

        let trie_iter = trie.iter();
        let ctrie_iter = ctrie.iter();

        let mut loaded = 0;

        for (trie_item, ctrie_item) in trie_iter.zip(ctrie_iter) {
            assert_eq!(
                trie_item, ctrie_item,
                "fail on iteration #{} - {:?} != cdatrie {:?}",
                loaded, trie_item, ctrie_item
            );
            loaded += 1;
        }

        assert_eq!(loaded, ctrie.iter().count());
    }
}

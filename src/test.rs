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

#[cfg(test)]
mod test {
    use crate::Trie;
    use std::path::PathBuf;

    #[test]
    fn load_libthai() {
        let path = PathBuf::from("/usr/share/libthai/thbrk.tri");

        if !path.exists() {
            println!("Skipping test as {:?} is missing", path);
            return;
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

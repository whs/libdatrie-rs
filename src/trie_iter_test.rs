use crate::testutils::{assert_dict_complete, en_trie_new, DICT};
use crate::types::{AlphaChar, AsAlphaChar};

// Ported from test_iterator.c
#[test]
fn test_iterator() {
    println!("Preparing trie");
    let mut trie = en_trie_new();

    println!("Adding data to trie");
    for word in DICT {
        assert!(
            trie.store(&word.as_alphachar(), 1),
            "Failed to store {}",
            word
        );
    }

    println!("Iterating and checking trie contents");
    assert_dict_complete(&trie);
}

// Ported from test_term_state.c
#[test]
fn test_term_state() {
    println!("Preparing trie");
    let mut trie = en_trie_new();

    println!("Populating trie with test set");
    assert!(
        trie.store(&"ab".as_alphachar(), 1),
        "Failed to add key 'ab' data 1"
    );
    assert!(
        trie.store(&"abc".as_alphachar(), 2),
        "Failed to add key 'abc' data 2"
    );

    let mut state = trie.root();
    println!("Try walking from root with 'a'");
    assert!(
        state.walk('a' as AlphaChar),
        "Failed to walk from root with 'a'"
    );
    assert_eq!(state.get_data(), None, "Retrieved data at 'a'");

    println!("Try walking further with 'b'");
    assert!(
        state.walk('b' as AlphaChar),
        "Failed to continue walking with 'b'"
    );
    assert_eq!(
        state.get_data(),
        Some(&1),
        "Retrieved data at 'ab' is not 1"
    );

    println!("Try walking further with 'c'");
    assert!(
        state.walk('c' as AlphaChar),
        "Failed to continue walking with 'c'"
    );
    assert_eq!(
        state.get_data(),
        Some(&2),
        "Retrieved data at 'abc' is not 2"
    );
}

// Ported from test_walk.c
#[test]
fn test_walk() {
    println!("Preparing trie");
    let mut trie = en_trie_new();

    // store
    println!("Adding data to trie");
    for word in ["pool", "prize", "preview", "prepare", "produce", "progress"] {
        assert!(
            trie.store(&word.as_alphachar(), 1),
            "Failed to store {}",
            word
        );
    }

    println!(
        r#"Now the trie structure is supposed to be:

          +---o-> (3) -o-> (4) -l-> [5]
          |
          |        +---i-> (7) -z-> (8) -e-> [9]
          |        |
(1) -p-> (2) -r-> (6) -e-> (10) -v-> (11) -i-> (12) -e-> (13) -w-> [14]
                   |         |
                   |         +---p-> (15) -a-> (16) -r-> (17) -e-> [18]
                   |
                   +---o-> (19) -d-> (20) -u-> (21) -c-> (22) -e-> [23]
                             |
                             +---g-> (24) -r-> (25) -e-> (26) -s-> (27) -s-> [28]
"#
    );

    // walk
    println!("Test walking");
    let mut s = trie.root();

    println!("Test walking with 'p'");
    assert!(s.is_walkable('p' as AlphaChar));
    assert!(s.walk('p' as AlphaChar));

    println!("Now at (2), walkable chars should be {{'o', 'r'}}");
    assert_eq!(s.walkable_chars(), vec!['o' as AlphaChar, 'r' as AlphaChar]);

    println!("Try walking from (2) with 'o' to (3)");
    let mut t = s.clone();
    assert!(
        t.walk('o' as AlphaChar),
        "Failed to walk from (2) with 'o' to (3)"
    );
    assert!(t.is_single(), "(3) should be single, but isn't");

    println!("Try walking from (3) with 'o' to (4)");
    assert!(
        t.walk('o' as AlphaChar),
        "Failed to walk from (3) with 'o' to (4)"
    );
    assert!(t.is_single(), "(4) should be single, but isn't");

    println!("Try walking from (4) with 'l' to (5)");
    assert!(
        t.walk('l' as AlphaChar),
        "Failed to walk from (4) with 'l' to (5)"
    );
    assert!(t.is_terminal(), "(5) should be terminal, but isn't");

    // get key & data
    println!("Try getting data from (5)");
    assert_eq!(t.get_data(), Some(&1), "Mismatched data from (5)");

    // walk s from (2) with 'r' to (6)
    println!("Try walking from (2) with 'r' to (6)");
    assert!(
        s.walk('r' as AlphaChar),
        "Failed to walk from (2) with 'r' to (6)"
    );

    println!("Now at (6), walkable chars should be {{'e', 'i', 'o'}}");
    assert_eq!(
        s.walkable_chars(),
        vec!['e' as AlphaChar, 'i' as AlphaChar, 'o' as AlphaChar]
    );

    // walk from s (6) with "ize"
    t = s.clone();
    println!("Try walking from (6) with 'i' to (7)");
    assert!(
        t.walk('i' as AlphaChar),
        "Failed to walk from (6) with 'i' to (7)"
    );

    println!("Try walking from (7) with 'z' to (8)");
    assert!(
        t.walk('z' as AlphaChar),
        "Failed to walk from (7) with 'z' to (8)"
    );
    assert!(t.is_single(), "(7) should be single, but isn't");

    println!("Try walking from (8) with 'e' to (9)");
    assert!(
        t.walk('e' as AlphaChar),
        "Failed to walk from (8) with 'e' to (9)"
    );
    assert!(t.is_terminal(), "(9) should be terminal, but isn't");

    println!("Try getting data from (5)");
    assert_eq!(t.get_data(), Some(&1), "Mismatched data from (9)");

    // walk from u = s (6) with 'e' to (10)
    println!("Try walking from (6) with 'e' to (10)");
    let mut u = s.clone();
    assert!(
        u.walk('e' as AlphaChar),
        "Failed to walk from (6) with 'e' to (10)"
    );

    // walkable chars from (10) should be {'p', 'v'}
    println!("Now at (10), walkable chars should be {{'p', 'v'}}");
    assert_eq!(u.walkable_chars(), vec!['p' as AlphaChar, 'v' as AlphaChar]);

    // walk from u (10) with "view"
    println!("Try walking from (10) with 'v' to (11)");
    t = u.clone();
    assert!(
        t.walk('v' as AlphaChar),
        "Failed to walk from (10) with 'v' to (11)"
    );
    assert!(t.is_single(), "(11) should be single, but isn't.");
    println!("Try walking from (11) with 'i' to (12)");
    assert!(
        t.walk('i' as AlphaChar),
        "Failed to walk from (11) with 'i' to (12)"
    );
    println!("Try walking from (12) with 'e' to (13)");
    assert!(
        t.walk('e' as AlphaChar),
        "Failed to walk from (12) with 'e' to (13)"
    );
    println!("Try walking from (13) with 'w' to (14)");
    assert!(
        t.walk('w' as AlphaChar),
        "Failed to walk from (13) with 'w' to (14)"
    );
    assert!(t.is_terminal(), "(14) should be terminal, but isn't");

    println!("Try getting data from (14)");
    assert_eq!(t.get_data(), Some(&1), "Mismatched data from (14)");

    // walk from u (10) with "pare"
    println!("Try walking from (10) with 'p' to (15)");
    t = u.clone();
    assert!(
        t.walk('p' as AlphaChar),
        "Failed to walk from (10) with 'p' to (15)"
    );
    assert!(t.is_single(), "(15) should be single, but isn't.");
    println!("Try walking from (15) with 'a' to (16)");
    assert!(
        t.walk('a' as AlphaChar),
        "Failed to walk from (15) with 'a' to (16)"
    );
    println!("Try walking from (16) with 'r' to (17)");
    assert!(
        t.walk('r' as AlphaChar),
        "Failed to walk from (16 with 'r' to (17)"
    );
    println!("Try walking from (17) with 'e' to (18)");
    assert!(
        t.walk('e' as AlphaChar),
        "Failed to walk from (17) with 'e' to (18)"
    );
    assert!(t.is_terminal(), "(18) should be terminal, but isn't");

    println!("Try getting data from (18)");
    assert_eq!(t.get_data(), Some(&1), "Mismatched data from (18)");

    // walk s from (6) with 'o' to (19)
    println!("Try walking from (6) with 'o' to (19)");
    assert!(
        s.walk('o' as AlphaChar),
        "Failed to walk from (6) with 'o' to (19)"
    );

    println!("Now at (19), walkable chars should be {{'d', 'g'}}");
    assert_eq!(s.walkable_chars(), vec!['d' as AlphaChar, 'g' as AlphaChar]);

    // walk from s (19) with "duce"
    println!("Try walking from (19) with 'd' to (20)");
    t = s.clone();
    assert!(
        t.walk('d' as AlphaChar),
        "Failed to walk from (19) with 'd' to (20)"
    );
    assert!(t.is_single(), "(20) should be single, but isn't.");
    println!("Try walking from (20) with 'u' to (21)");
    assert!(
        t.walk('u' as AlphaChar),
        "Failed to walk from (20) with 'u' to (21)"
    );
    println!("Try walking from (21) with 'c' to (22)");
    assert!(
        t.walk('c' as AlphaChar),
        "Failed to walk from (21) with 'c' to (22)"
    );
    println!("Try walking from (22) with 'e' to (23)");
    assert!(
        t.walk('e' as AlphaChar),
        "Failed to walk from (22) with 'e' to (23)"
    );
    assert!(t.is_terminal(), "(23) should be terminal, but isn't");

    println!("Try getting data from (23)");
    assert_eq!(t.get_data(), Some(&1), "Mismatched data from (23)");

    // walk from s (19) with "gress"
    println!("Try walking from (19) with 'g' to (24)");
    assert!(
        s.walk('g' as AlphaChar),
        "Failed to walk from (19) with 'g' to (24)"
    );
    assert!(s.is_single(), "(24) should be single, but isn't.");
    println!("Try walking from (24) with 'r' to (25)");
    assert!(
        s.walk('r' as AlphaChar),
        "Failed to walk from (24) with 'r' to (25)"
    );
    println!("Try walking from (25) with 'e' to (26)");
    assert!(
        s.walk('e' as AlphaChar),
        "Failed to walk from (25) with 'e' to (26)"
    );
    println!("Try walking from (26) with 's' to (27)");
    assert!(
        s.walk('s' as AlphaChar),
        "Failed to walk from (26) with 's' to (27)"
    );
    println!("Try walking from (27) with 's' to (28)");
    assert!(
        s.walk('s' as AlphaChar),
        "Failed to walk from (27) with 's' to (28)"
    );
    assert!(s.is_terminal(), "(28) should be terminal, but isn't");

    println!("Try getting data from (28)");
    assert_eq!(s.get_data(), Some(&1), "Mismatched data from (28)");
}

use serial_test::serial;

extern "C" {
    fn dict_reset();
    fn c_test_byte_alpha() -> i32;
    fn c_test_byte_list() -> i32;
    fn c_test_file() -> i32;
    fn c_test_iterator() -> i32;
    fn c_test_nonalpha() -> i32;
    fn c_test_null_trie() -> i32;
    fn c_test_serialization() -> i32;
    fn c_test_store_retrieve() -> i32;
    fn c_test_term_state() -> i32;
    fn c_test_walk() -> i32;
}

#[test]
#[serial(ctest)]
fn test_byte_alpha() {
    assert_eq!(unsafe { c_test_byte_alpha() }, 0);
}

#[test]
#[serial(ctest)]
fn test_byte_list() {
    assert_eq!(unsafe { c_test_byte_list() }, 0);
}

#[test]
#[serial(ctest)]
fn test_file() {
    unsafe { dict_reset() }
    assert_eq!(unsafe { c_test_file() }, 0);
}

#[test]
#[serial(ctest)]
fn test_iterator() {
    assert_eq!(unsafe { c_test_iterator() }, 0);
    unsafe { dict_reset() }
}

#[test]
#[serial(ctest)]
fn test_nonalpha() {
    assert_eq!(unsafe { c_test_nonalpha() }, 0);
    unsafe { dict_reset() }
}

#[test]
#[serial(ctest)]
fn test_null_trie() {
    assert_eq!(unsafe { c_test_null_trie() }, 0);
}

#[test]
#[serial(ctest)]
fn test_serialization() {
    unsafe { dict_reset() }
    assert_eq!(unsafe { c_test_serialization() }, 0);
}

#[test]
#[serial(ctest)]
fn test_store_retrieve() {
    unsafe { dict_reset() }
    assert_eq!(unsafe { c_test_store_retrieve() }, 0);
}

#[test]
#[serial(ctest)]
fn test_term_state() {
    assert_eq!(unsafe { c_test_term_state() }, 0);
}

#[test]
#[serial(ctest)]
fn test_walk() {
    unsafe { dict_reset() }
    assert_eq!(unsafe { c_test_walk() }, 0);
}
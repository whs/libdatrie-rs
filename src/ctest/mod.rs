use inline_c::assert_c;
use serial_test::serial;

#[test]
#[serial(ctest)]
fn test_byte_alpha() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_byte_alpha.c"

        int main (void)
        {
            return c_test_byte_alpha ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_byte_list() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_byte_list.c"

        int main (void)
        {
            return c_test_byte_list ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_file() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_file.c"

        int main (void)
        {
            dict_reset ();
            return c_test_file ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_iterator() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_iterator.c"

        int main (void)
        {
            int result = c_test_iterator ();
            dict_reset ();
            return result;
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_nonalpha() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_nonalpha.c"

        int main (void)
        {
            int result = c_test_nonalpha ();
            dict_reset ();
            return result;
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_null_trie() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_null_trie.c"

        int main (void)
        {
            return c_test_null_trie ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_serialization() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_serialization.c"

        int main (void)
        {
            dict_reset ();
            return c_test_serialization ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_store_retrieve() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_store-retrieve.c"

        int main (void)
        {
            dict_reset ();
            return c_test_store_retrieve ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_term_state() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_term_state.c"

        int main (void)
        {
            return c_test_term_state ();
        }
    })
    .success();
}

#[test]
#[serial(ctest)]
fn test_walk() {
    (assert_c! {
        #include "../../../src/ctest/utils.c"
        #include "../../../src/ctest/test_walk.c"

        int main (void)
        {
            dict_reset ();
            return c_test_walk ();
        }
    })
    .success();
}

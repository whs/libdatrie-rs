# Datrie reimplementation in Rust

datrie is a [libdatrie](https://linux.thai.net/~thep/datrie/datrie.html) reimplementation, capable of loading libdatrie files.

This is in pre-alpha stage

## Feature flags

* cffi: Enable building of C binding. Without it the C comparison test will run (default on)

## Available FFI features

The FFI is intended to be a drop in replacement for original libdatrie, except for unexported functions.

| Function                  | Available       |
|---------------------------|-----------------|
| alpha_map_new             | ✅️              |
| alpha_map_clone           | ✅️              |
| alpha_map_free            | ✅️              |
| alpha_map_add_range       | ✅️              |
| alpha_char_strlen         | ✅️              |
| trie_new                  | ✅️              |
| trie_new_from_file        | ✅️              |
| trie_free                 | ✅️              |
| trie_save                 | ✖️              |
| trie_is_dirty             | ✅️              |
| trie_retrieve             | ✅️             |
| trie_store                | 💣️             |
| trie_delete               | ✖️              |
| trie_enumerate            | ✅️              |
| trie_root                 | ✅️              |
| trie_state_clone          | ✅️              |
| trie_state_copy           | ✅️              |
| trie_state_free           | ✅️              |
| trie_state_rewind         | ✖️              |
| trie_state_walk           | 💣️             |
| trie_state_is_walkable    | 💣️             |
| trie_state_is_single      | ✅️              |
| trie_state_get_data       | ✅️              |
| trie_store_if_absent      | ✅️              |
| trie_fread                | ✅️ (Unix only) |
| trie_fwrite               | ✖️              |
| trie_state_walkable_chars | 💣️             |
| trie_iterator_new         | ✅️             |
| trie_iterator_free        | ✅️              |
| trie_iterator_next        | ✅️              |
| trie_iterator_get_key     | ✅️             |
| trie_iterator_get_data    | ✅️              |
| alpha_char_strcmp         | ✅️              |
| trie_get_serialized_size  | ✖️              |
| trie_serialize            | ✖️              |

## License
As a derivative work of libthai, this package is licensed under the [LGPL 2.1](LICENSE) license.

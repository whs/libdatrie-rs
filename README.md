# Datrie reimplementation in Rust

datrie is a [libdatrie](https://linux.thai.net/~thep/datrie/datrie.html) ported with C2Rust and rewritten into safe Rust.

## Feature flags

* cffi: Enable building of C binding. Without it the C comparison test will run (default on)

## Available FFI features

The FFI is intended to be a drop in replacement for original libdatrie, except for unexported functions.

## License
As a derivative work of libthai, this package is licensed under the [LGPL 2.1](LICENSE) license.

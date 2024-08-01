# Datrie reimplementation in Rust

datrie is a [libdatrie](https://linux.thai.net/~thep/datrie/datrie.html) ported with C2Rust and rewritten into safe Rust.

This project is intended to be a drop in replacement for original libdatrie, except for unexported functions. It has
been tested with original libdatrie's test.

Using this library in Rust is possible, but in many places it expects and returns null terminated arrays

## Feature flags

* cffi: Enable building of C binding. Without it the C comparison test will run (default on)

## On speed

It is currently unclear whether this library is faster or slower than the original library:

**Pros**

* All data structure are stack-allocated, reducing memory allocation. Of course, this excludes arrays and types passing
  through the FFI boundary.
* With generic and inlining, Rust can generate more optimal code than C, which might not inline exported
  function (even if exported internally) - at least on how it was used to compile original libdatrie.

**Cons**

* All array size are computed in advance (require a loop) while the C version sometimes do not check array size
* All array access are bound checked which create overhead
* Some memory access in C use uninitialized memory, but only if the user call things in the wrong order. In the Rust
  version all data are either initialized (adding initialization cost) or behind `Option<>` (adding additional checks).

## License
As a derivative work of libdatrie, this package is licensed under the [LGPL 2.1](LICENSE) license.

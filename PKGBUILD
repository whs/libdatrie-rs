pkgname=(libdatrie-rs libdatrie-rs-utils)
pkgver=1.0.0
pkgrel=1
pkgdesc="Rust port of libdatrie"
arch=('x86_64')
url="https://github.com/whs/libthairs/blob/master/datrie/"
license=('LGPL-2.1-or-later')
depends=('gcc-libs')
makedepends=('cargo')
provides=('libdatrie')
conflicts=('libdatrie')
source=('datrie-1.0.0.tar.gz')
b2sums=('SKIP')

prepare() {
    export RUSTUP_TOOLCHAIN=nightly
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  export RUSTUP_TOOLCHAIN=nightly
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --features bin
}

check() {
  export RUSTUP_TOOLCHAIN=nightly
  cargo test --frozen --features bin
}

package_libdatrie-rs() {
  install -Dm0755 "target/release/libdatrie.so" "$pkgdir/usr/lib/libdatrie.so.1.4.0"
  install -Dm0644 "trie.h" "$pkgdir/usr/include/datrie/trie.h"
  ln -s "libdatrie.so.1.4.0" "$pkgdir/usr/lib/libdatrie.so"
  ln -s "libdatrie.so.1.4.0" "$pkgdir/usr/lib/libdatrie.so.1"
}

package_libdatrie-rs-utils() {
  pkgdesc="Rust port of libdatrie. This package contains trietool utility."
  depends=('gcc-libs' 'libdatrie-rs')
  provides=()
  conflicts=()

  install -Dm0755 "target/release/trietool" "$pkgdir/usr/bin/trietool"
}

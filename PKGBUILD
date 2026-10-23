pkgname=(libdatrie-rs libdatrie-rs-utils)
pkgver=1.0.0
pkgrel=2
pkgdesc="Rust port of libdatrie"
arch=('x86_64')
url="https://github.com/whs/libthairs/blob/master/datrie/"
license=('LGPL-2.1-or-later')
depends=('gcc-libs')
makedepends=('cargo' 'cargo-c')
provides=('libdatrie')
conflicts=('libdatrie')
source=('datrie-1.0.0.tar.gz')
b2sums=('SKIP')

prepare() {
  cargo fetch --locked
}

build() {
  cargo cbuild --frozen --release
  cargo build --frozen --release --features bin
}

check() {
  cargo ctest --frozen --release -F ctest
}

package_libdatrie-rs() {
  cargo cinstall --frozen --release --destdir="$pkgdir" --prefix=/usr --libdir=lib
}

package_libdatrie-rs-utils() {
  pkgdesc="Rust port of libdatrie. This package contains trietool utility."
  depends=('gcc-libs' 'libdatrie-rs')
  provides=()
  conflicts=()

  install -Dm0755 "target/release/trietool" "$pkgdir/usr/bin/trietool"
}

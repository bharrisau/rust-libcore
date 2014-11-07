#!/bin/sh
mkdir -p rust
cd rust
wget -q -O rust.tar.gz https://github.com/rust-lang/rust/tarball/8ed288edb
tar -zx --strip-components=1 -f rust.tar.gz

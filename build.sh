#!/bin/sh
mkdir -p rust
cd rust
wget -q -O rust.tar.gz https://github.com/rust-lang/rust/tarball/`rustc --version|awk '{sub(/\\(/, "", $3); print $3}'`
tar -zx --strip-components=1 -f rust.tar.gz

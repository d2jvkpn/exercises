#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cat > Cargo.toml <<EOF
[workspace]
resolver = "2"

members = []
EOF

cargo install cargo-modules

cargo new p1
cargo new --lib utils

cat Cargo.toml

cargo run --bin p1
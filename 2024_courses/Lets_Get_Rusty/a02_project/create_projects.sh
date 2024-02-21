#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
cat > Cargo.toml <<EOF
[workspace]
resolver = "2"

members = []
EOF

cargo install cargo-modules bench
cargo modules structure
cargo modules structure --package auth_service 

cargo new --bin auth_service
cargo new --lib utils

cargo build -p auth_service

cat Cargo.toml
cargo run --bin auth_service

# cargo login xxxxxxxx
# cd auth_service
# git add -A
# git commit -am 'init'
# cargo publish

cargo new --lib draw
cd draw

cargo add serde --features=derive --optional
cargo add rgb --features=serde --optional

#! /usr/bin/env bash
set -eu -o pipefail

_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})
# set -x

# echo "Hello, world!"

# cargo new p01 && cd p01

cargo add tokio@1.35 --features=full
cargo add reqwest@0.11 --features=json
cargo add serde@1 --features=derive
cargo add chrono@0.4 serde_json@1

echo 'use_small_heuristics = "Max"' > rustfmt.toml

sed -i '/#/i default-run = "p01"' Cargo.toml

cat >> Cargo.toml <<EOF 
[[bin]]
name = "a01"
path = "bin/a01.rs"

[[bin]]
name = "a02"
path = "bin/a02.rs"
EOF

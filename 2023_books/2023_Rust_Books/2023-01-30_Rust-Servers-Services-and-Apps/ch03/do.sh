#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


####
# project ezytutors
cargo new ezytutors && cd ezytutors

cat >> .rustfmt.toml <<EOF
use_small_heuristics = "Max"
EOF

cat >> Cargo.toml <<EOF

[workspace]
members = ["tutor-nodb"]
EOF


# project ezytutors/tutor-nodb
cargo new tutor-nodb && cd tutor-nodb

cargo add actix-web@4 actix-rt@2

cat >> Cargo.toml <<EOF

[[bin]]
name = "basic-server"
EOF

mkdir -p src/bin
touch src/bin/basic-server.rs
# coding...

cargo run --bin basic-server

cargo add chrono@0.4 --feaures=serde
cargo add serde@1 --feaures=derive

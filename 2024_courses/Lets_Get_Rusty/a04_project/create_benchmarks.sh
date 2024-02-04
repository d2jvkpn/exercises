#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


cd utils

cargo add --dev criterion

cat >> Cargo.toml <<EOF
[[bench]]
name = "sorting_benchmark"
harness = false
EOF

cargo install bench
cargo bench

ls target/criterion/report/index.html

#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


cargo fmt
cargo clippy
cargo check

cargo doc --no-deps --open

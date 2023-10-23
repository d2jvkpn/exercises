#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo install cargo-update
cargo install cargo-edit

cargo new my-project
cd my-project

cargo build
cargo build --release
cargo run

cargo add serde@1
cargo remove serde

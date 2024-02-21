#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# cargo install cargo-expand

cd procedural
cargo expand --bin procedural

cd declarative
cargo expand --bin declarative

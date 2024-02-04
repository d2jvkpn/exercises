#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


cd utils
# all tests
cargo test

cargo test --lib
cargo test --doc
cargo test --tests t_savings_account

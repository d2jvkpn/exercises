#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo test --lib -- "my_module::tests::t_add" --nocapture

cargo test --test "my_module_test" -- --nocapture

cargo test --test integration_test

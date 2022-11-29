#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo test --bins
cargo test --bins -- plus
cargo test --bins -- plus --show-output
cargo test --bins -- t_plus --exact # not run any test
cargo test --bins -- tests::t_plus --exact

cargo test --lib

cargo test --test "*" # tests in tests/

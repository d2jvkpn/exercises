#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### bins
# default-run = "ch01"
cargo run

cargo run --bin second

cargo run --bin third


#### deps
cargo add chrono@0.4 --features=serde

cargo run

#### tests
# run tests in lib, bins, and tests/
cargo test

# run tests in lib only
cargo test --lib -- hello_lib --show-output

# run tests in docs
cargo test --doc

# run integration tests
cargo test --test '*'

cargo test --test '*' -- --ignored

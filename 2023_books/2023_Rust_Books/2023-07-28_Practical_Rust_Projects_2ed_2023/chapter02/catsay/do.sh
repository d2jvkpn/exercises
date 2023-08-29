#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#### add packages
cargo add clap@4 --feaures=derive
cargo add colored

cargo add --dev assert_cmd predicates

#### library test
cargo test --lib -- --nocapture

#### binary test
cargo test --bins -- --nocapture

#### integration_test
cargo test --test "*"

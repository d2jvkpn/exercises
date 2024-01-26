#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo add chrono@0.4 --features=serde
cargo add env_logger
cargo add log --features=std,serde
cargo add --dev criterion

cargo bench
ls target/criterion/report/index.html

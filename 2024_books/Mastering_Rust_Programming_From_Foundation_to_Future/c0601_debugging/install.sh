#! /usr/bin/env bash
set -eu -o pipefail

_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})
# set -x

cargo install flamegraph
cargo install cargo-profiler

cargo add chrono@0.4 --features=serde
cargo add env_logger
cargo add log --features=std,serde

cargo add --dev criterion

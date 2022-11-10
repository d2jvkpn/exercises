#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


cargo add anyhow thiserror

## tests in libraries
cargo test --lib

## tests in main.rs
cargo test --bins -- tests::t_42 --exact --show-output

## unit tests in /tests
cargo test --test "*" -- --show-output

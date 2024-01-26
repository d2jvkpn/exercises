#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo build --target=x86_64-pc-windows-gnu
ls target/x86_64-pc-windows-gnu/debug/c1001.exe

cargo build
ls target/debug/c1001

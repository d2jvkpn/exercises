#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

mkdir -p target

gcc 11_sockets.c -o target/11_sockets

strace -f target/11_sockets

#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

cargo test --lib -- t_kth_smallest --show-output
# --doc, --bins

cargo test --lib -- tests::t_kth_smallest --exact --show-output

#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


file=$1
sed 's/\xc2\xa0/ /g' -i $file

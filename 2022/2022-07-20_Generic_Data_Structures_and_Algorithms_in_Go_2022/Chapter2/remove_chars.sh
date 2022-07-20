#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

input=$1

sed -ir 's: : :g' $input

go fmt $input
go vet $input

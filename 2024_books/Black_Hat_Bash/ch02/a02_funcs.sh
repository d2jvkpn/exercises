#!/usr/bin/env bash
# set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

function print_args(){
    echo "Args: $*"
    echo "First: ${1}, Second: ${2}, Third: ${3}"
    
    for v in "$@"; do
        echo "~~~ $v"
    done
}

print_args No Starch "Press xxxx"

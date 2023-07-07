#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

mkdir -p target

C_FILE=$1
bin=$(basename $C_FILE | sed 's/\.c$//')

gcc ${C_FILE} -o target/$bin -l ncurses
./target/$bin

#!/usr/bin/env bash
# set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})


echo $#
echo $@
echo $*
echo $0
echo $1, $2

for v in $@; do
    echo "~~~ $v"
done

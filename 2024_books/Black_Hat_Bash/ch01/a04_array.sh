#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

arr=(hello world "yes ok" must)

echo ${arr[1]}, ${arr[2]}

echo ${arr#}

echo ${arr[2]}
echo ${arr[@]:1} # arra[1:]

echo ${arr[@]:1:3} # arra[1:3]


echo ${arr[@]: -2:1} # arr[-2:1] # the first element from -2
echo ${arr[@]: -2:2} # arr[-2:2] # the last two elements

for v in ${arr[@]}; do echo $v; done
for v in "${arr[@]}"; do echo $v; done

uset ${arr[1]}

#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### echo
echo "Test echo" > file.txt
echo "This is Linux" >> file.txt

cat file.txt


#### ls
ls -al
ls -hs
man ls

clear

whoami

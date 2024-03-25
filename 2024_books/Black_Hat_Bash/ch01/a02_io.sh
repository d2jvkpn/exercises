#!/usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

ls -1 / &> stdout_and_stderr.txt

ls -l / 1> stdout.txt 2> stderr.txt

cat < output.txt

cat << EOF
Black Hat Bash
by No Starch Press
EOF

ls -l / | grep "bin"


&>2 echo "Hello" 2> hello.error

read -t 10 -p "What's your name? " name
echo "Hello, $name"

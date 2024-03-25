#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

####
sleep 100 &

ps -ef | grep sleep

jobs

fg %1
# Ctrl + Z to make the process stopped

bg %1

kill %1

####
nohup ./my_script.sh &

#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#### ncurses
../C_Run.sh 05_ncurses1.c -l ncurses

#### sockets
ping www.baidu.com

host www.baidu.com

nslookup www.baidu.com

#### strace
mkdir -p target

gcc 11_sockets.c -o target/11_sockets

strace -f target/11_sockets

#### svc
netstat -anlp | head

env - telnet localhost 8000


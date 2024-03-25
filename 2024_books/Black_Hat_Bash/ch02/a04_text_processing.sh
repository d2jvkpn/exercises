#! /usr/bin/env bash
set -eu -o pipefail # -x
_wd=$(pwd); _path=$(dirname $0 | xargs -i readlink -f {})

####
grep "35.237.4.214\|13.66.139.0" log.txt
grep -e "35.237.4.214" -e "13.66.139.0" log.txt

ps | grep TTY
ps | grep -i tty

ps | grep -v TTY

grep -o "35.237.4.214" log.txt

####
sed 's/Mozilla/Godzilla/g' log.txt
sed 's/ //g' log.txt

sed '1d' log.txt
sed '$d' log.txt
sed '5,7d' log.txt

sed -n '2,15 p' log.txt

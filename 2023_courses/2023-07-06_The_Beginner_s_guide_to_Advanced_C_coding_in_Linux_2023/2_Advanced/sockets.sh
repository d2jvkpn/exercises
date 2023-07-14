#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

ping www.baidu.com

host www.baidu.com

nslookup www.baidu.com

#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### 01
addr=http://localhost:8080
curl -i $addr
curl -i $addr/maxwell
curl -i $addr/say/hello

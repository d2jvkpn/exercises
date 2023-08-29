#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


curl localhost:8080/metrics

curl localhost:8080/hash -d test &
curl -s localhost:8080/metrics | grep inflight

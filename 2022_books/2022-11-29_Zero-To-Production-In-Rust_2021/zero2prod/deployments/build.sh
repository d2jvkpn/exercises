#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# --network=host
docker build --no-cache -f ${_path}/build.df --tag zero2prod:dev ./

docker image prune --force --filter label=stage=zero2prod_builder &> /dev/null

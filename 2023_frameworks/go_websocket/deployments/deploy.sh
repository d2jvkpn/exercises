#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


BuildBranch="$1"
APP_ENV="$2"
PORT=$3

export APP_ENV=${APP_ENV} BuildBranch=${BuildBranch} PORT=${PORT}
envsubst < ${_path}/deploy.yaml > docker-compose.yaml

docker-compose pull
[[ ! -z "$(docker ps --all --quiet --filter name=hello_$APP_ENV)" ]] && docker rm -f hello_$APP_ENV
# docker-compose down for running containers only, not stopped containers

docker-compose up -d

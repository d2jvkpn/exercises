#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

docker run -p=6379:6379 --name=redis_db -d --restart=always redis:7-alpine \
  redis-server --save 60 1 --loglevel warning
# /usr/local/etc/redis/redis.conf 

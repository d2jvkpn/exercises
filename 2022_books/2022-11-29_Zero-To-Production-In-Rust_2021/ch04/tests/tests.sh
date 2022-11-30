#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

curl -i -X GET localhost:8000/healthy

curl -i -X POST localhost:8000/subscribe \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d 'name=le%20guin&email=ursula_le_guin%40gmail.com'

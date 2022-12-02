#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


####
addr=http://localhost:8081

curl -i $addr/demo/healthy

curl -i localhost:8081/demo/index

curl -i -X POST -H "Content-Type: application/json" $addr/demo/welcome \
  -d '{}'

curl -i -X POST -H "Content-Type: application/json" $addr/demo/welcome \
  -d '{"name": "Rover"}'

curl -i -X POST -H "Content-Type: application/json" -H "X-Platform: web" \
  $addr/demo/hello/English?id=42 \
  -d '{"name": "Rover"}'


####
addr=http://localhost:8080

curl -i -X POST -H "Content-Type: application/json" $addr/api/welcome \
  -d '{}'

curl -i -X POST -H "Content-Type: application/json" $addr/api/open/login/h5 \
  -H "X-Language: English" \
  -d '{"email": "xxxx@gmail.com", "password": "yyyy"}'

curl -i -X POST -H "Content-Type: application/json" $addr/api/auth/logout

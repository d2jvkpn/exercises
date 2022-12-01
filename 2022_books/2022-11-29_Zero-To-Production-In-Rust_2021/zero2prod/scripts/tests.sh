#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### cargo test
cargo test --lib

cargo test --lib -- common --show-output

cargo test --bins

cargo test --test "*" -- health_check --exact

#### test by using curl
addr=http://localhost:8000

curl -i -X GET $addr/healthz

curl -i -X GET $addr/v1/healthy

curl -i -X GET $addr/open/info/42/Mason

curl -i -X GET "$addr/open/greet/Evol?pageNo=1&pageSize=100"

curl -i -X POST $addr/open/subscribe \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d 'name=le%20guin&email=ursula_le_guin%40gmail.com'

curl -i -X POST -H "Content-Type: application/json" \
  $addr/open/hello -d '{}'

curl -i -X POST -H "Content-Type: application/json" \
  $addr/open/hello -d '{"name": "Evol Mason"}'

curl -i -X POST -H "Content-Type: application/json" \
  $addr/open/hello?id=101 -d '{"name": "Evol Mason"}'

curl -i -X POST -H "Content-Type: application/json" \
  $addr/open/hello/h5 -d '{"name": "Evol Mason, Evol Mason, Evol Mason, Evol Mason, Evol Mason"}'

# plow "http://$addr/api/open/sign_in" -T 'application/json' -m POST \
#   -c 10 -d 30s --timeout 2s --body "@data.json"

plow $addr/healthz -c 100 -d 30s --timeout 2s

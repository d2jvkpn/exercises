#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# append records
curl -i -X POST localhost:8080 -d '{"record": {"value": "TGV0J3MgR28gIzEK"}}'

curl -i -X POST localhost:8080 -d '{"record": {"value": "TGV0J3MgR28gIzIK"}}'

curl -i -X POST localhost:8080 -d '{"record": {"value": "TGV0J3MgR28gIzMK"}}'

curl -i -X POST localhost:8080 -d '{"record": {"value": 1}}'

# read records
curl -i -X GET localhost:8080 -d '{"offset": 0}'

curl -i -X GET localhost:8080 -d '{"offset": 1}'

curl -i -X GET localhost:8080 -d '{"offset": 2}'

curl -i -X GET localhost:8080 -d '{"offset": 10}'

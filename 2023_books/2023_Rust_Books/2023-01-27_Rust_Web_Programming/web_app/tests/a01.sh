#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

addr=http://localhost:8080

#### 01
curl -i $addr/api/v1/open/greet
curl -i $addr/api/v1/open/maxwell
curl -i $addr/api/v1/open/say/hello

#### 02
curl -i -X POST $addr/api/v1/open/login
curl -i -X POST $addr/api/v1/auth/logout

#### 03
curl -i -X GET $addr/api/v1/item/get
curl -i -X POST $addr/api/v1/item/create/Rust

curl -i -X POST $addr/api/v1/item/edit \
  -H 'Content-Type: application/json'  \
  -H 'X-Token: ABC'  \
  -d '{"title": "Rust", "status": "DONE"}'

curl -i -X GET $addr/api/v1/item/get

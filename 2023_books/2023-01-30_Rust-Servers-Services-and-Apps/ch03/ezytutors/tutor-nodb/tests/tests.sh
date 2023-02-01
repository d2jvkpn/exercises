#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
curl -i -X POST -H "content-type: application/json" http://localhost:3000/courses/create \
  -d '{"tutorId": 10, "courseName": "Rust"}'

curl -i -X GET http://localhost:3000/courses/10

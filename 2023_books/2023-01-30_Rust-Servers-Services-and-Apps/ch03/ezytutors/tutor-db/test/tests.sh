#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


addr=http://localhost:3000
####
curl -i -X POST -H "content-type: application/json" $addr/courses/create \
  -d '{"courseId": 0, "tutorId": 10, "courseName": "Fourth course"}'

curl -i -X GET $addr/courses/10

curl -i -X GET $addr/courses/10/9

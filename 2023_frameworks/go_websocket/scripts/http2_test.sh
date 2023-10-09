#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#### one tcp connection, two requests
req="GET /api/open/ping HTTP/1.1\r\nHost: github.com\r\n\r\n"
echo -e "$req$req" | netcat -i 1 127.0.0.1 8080

req="GET /api/open/ping HTTP/1.1\nHost: github.com\n\n"
echo -e "$req$req" | netcat 127.0.0.1 8080

####
curl -X POST http://localhost:8080/web/open/upload \
  -F "file=@./scripts/demo.msh" -H "Content-Type: multipart/form-data"

curl -X POST http://localhost:8080/web/upload \
  -F "upload[]=@./main"             \
  -F "upload[]=@./scripts/demo.msh" \
  -H "Content-Type: multipart/form-data"

curl -X POST http://localhost:8080/web/upload \
  -F "upload[]=@./scripts/demo.msh" \
  -H "Content-Type: multipart/form-data"

curl http://localhost:8080/web/static/upload/demo.msh

####
mkdir -p .tmp
touch .tmp/a.txt

curl -X POST http://localhost:8080/web/upload \
  -F "upload[]=@.tmp/a.txt"                      \
  -H "Content-Type: multipart/form-data"

####
curl -X POST http://k8scp/web/upload \
  -F "upload[]=@./scripts/demo.msh"  \
  -H "Content-Type: multipart/form-data"

curl http://k8scp/web/static/upload/demo.msh

# use k8scp.local in browser
curl http://k8scp.local/web/static/upload/demo.msh

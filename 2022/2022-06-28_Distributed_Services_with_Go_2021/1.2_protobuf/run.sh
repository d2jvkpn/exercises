#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


#
wget https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-x86_64.zip

unzip protoc-21.2-linux-x86_64.zip -d /opt/protoc


go get google.golang.org/protobuf/...@v1.25.0

#
mkdir -p api/v1/

cat > api/v1/log.proto << EOF
syntax = "proto3";
package log.v1;

option go_package = "prolog/api/log_v1";

message Record {
  bytes value = 1;
  uint64 offset = 2;
}
EOF

cat api/v1/log.proto

protoc api/v1/*.proto --go_out=. --go_opt=paths=source_relative --proto_path=.

cat api/v1/log.pb.go

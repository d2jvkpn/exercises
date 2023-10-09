#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

apt install -y protobuf-compiler

go get -u google.golang.org/grpc
go get -u github.com/golang/protobuf/{proto,protoc-gen-go}
go get -u google.golang.org/grpc/cmd/protoc-gen-go-grpc
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc

# protoc greet/greet/greet.proto --go_out==plugins=grpc:./
# protoc greet/greet/greet.proto --go-grpc_out=.

# protoc --go_out=plugins=grpc:. pkg/greet/greet.proto

protoc --go_out=./  --go-grpc_out=./  pkg/greet/greet.proto
# comment greet_grpc.pb.go::GreetServiceServer/mustEmbedUnimplementedGreetServiceServer()
# generate: greet.pb.go, greet_grpc.pb.go

sed -i '/^\tmustEmbedUnimplemented/s#\t#\t// #' pkg/greet/greet_grpc.pb.go

ls -al data/

truncate -s 10m data/grpc_send.data

md5sum data/grpc_send.data data/grpc_recevied.data

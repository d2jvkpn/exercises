#! /usr/bin/env bash
set -eu -o pipefail
# set -x
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev

# sudo apk add protoc protobuf-dev

cargo new auth
cd auth

cargo add tonic prost
cargo add tokio --features=full
cargo add --build tonic-build

mkdir -p proto

cat > proto/hello.proto <<EOF
syntax = "proto3";
package hello;

service Greeter {
    rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
   string name = 1;
}

message HelloReply {
    string message = 1;
}
EOF

cat > build.rs <<EOF 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;
    Ok(())
}
EOF

# cargo install protoc-gen-tonic protoc-gen-prost
# mkdir -p proto/gen
# protoc -I proto proto/hello.proto --prost_out=proto/gen --tonic_out=proto/gen

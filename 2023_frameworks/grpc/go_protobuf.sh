#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### install tools
mkdir -p ~/Apps

cat >> .bash_aliases <<EOF
#### add ~/Apps
for d in $(ls -d ~/apps/*/ 2>/dev/null); do
    d=${d%/}
    if [ -d $d/bin ]; then d=$d/bin; fi
    export PATH=$d:$PATH
done
EOF

version=23.2

wget -P ~/Downloads https://github.com/protocolbuffers/protobuf/releases/download/v${version}/protoc-${version}-linux-x86_64.zip

mkdir -p ~/Apps/protoc-${version}
unzip ~/Downloads/protoc-${version}-linux-x86_64.zip -d ~/Apps/protoc-${version}

go get google.golang.org/protobuf@v1.30.0
# go get -u github.com/golang/protobuf/{proto,protoc-gen-go}@v1.30.0
go get -u google.golang.org/grpc

go get -u google.golang.org/grpc/cmd/protoc-gen-go-grpc
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc

# go get google/protobuf/timestamp.proto

#### create proto
mkdir -p proto

cat > proto/log.proto << EOF
syntax = "proto3";
package proto;

// option go_package = "github.com/d2jvkpn/app/proto";

// import "google/protobuf/timestamp.proto";

message LogData {
	string name = 1;
	string version = 2;
	string eventId = 3;
	int64  eventAt = 4;
	// google.protobuf.Timestamp eventAt = 4;
	string bizName = 5;
	string bizVersion = 6;
	map<string, string> keys = 7;
	bytes data = 8;
}

message LogId {
	string id = 1;
}

service LogService {
	rpc Create(LogData) returns(LogId) {};
}
EOF

#### grpc generate
protoc proto/*.proto --go_out=plugins=grpc:. --go_opt=paths=source_relative --proto_path=.
ls -al proto/

go fmt ./...
go vet ./...

####
# implment...

####
# sed -i '/^\tmustEmbedUnimplemented/s#\t#\t// #' proto/log_data.pb.go

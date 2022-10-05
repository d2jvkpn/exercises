#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
go test -run TestMe

go test -run TestMe -v

go test -v -run /test1 # run t.Run("test1", test1)

####
go test -v -bench .

go test --run none -bench ^BenchmarkSum$ -v

go test --run none -bench BenchmarkSumParallel -cpu 1,4,16

####
go test -run TestOptions -v -cover

go test -run TestOptions -v -cover -coverprofile=TestOptions

go tool cover -func=TestOptions

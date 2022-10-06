#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### testflag
go help testflag

#### test
go test -run TestMe

go test -run TestMe -v

go test -v -run /test1 # run t.Run("test1", test1)

#### benchmark
go test -v -bench .

go test --run none -bench ^BenchmarkSum$ -v

go test --run none -bench BenchmarkSumParallel -cpu 1,4,16 -count 5 -parallel 10

#### coverage
go test -run TestOptions -v -cover

go test -run TestOptions -v -cover -coverprofile=TestOptions

go tool cover -func=TestOptions

#### pprof
go test --run none -bench=BenchmarkGraph \
  -benchmem \
  -memprofile=mem.pprof \
  -cpuprofile=cpu.pprof

go tool pprof -pdf -output cpu.pprof.pdf cpu.pprof

go tool pprof -pdf -output mem.pprof.pdf mem.pprof

go tool pprof cpu.pprof # interactive mode commands:
# help
# top
# list BuildGraph
# q

go tool pprof mem.pprof

#### doc
go doc -all
go doc Periods

go doc -src Periods

godoc -http=:8080

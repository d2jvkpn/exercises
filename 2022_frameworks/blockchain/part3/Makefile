gitBranch = $(shell git rev-parse --abbrev-ref HEAD)
gitTime = $(shell date +'%FT%T%:z')

build:
	mkdir -p target
	GOOS=linux   GOARCH=amd64 go build -o target/main -ldflags="-w -s" ./
	ls -lh target/

wasm_path = $(shell go env GOROOT)/misc/wasm

prepare:
	cp ${wasm_path}/wasm_exec.js ./

build:
	GOOS=js GOARCH=wasm go build -o main.wasm main.go

run:
	GOOS=js GOARCH=wasm go build -o main.wasm main.go
	live-server

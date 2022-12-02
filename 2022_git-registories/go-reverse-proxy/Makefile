build:
	mkdir -p target
	GOOS=linux   GOARCH=amd64 go build -o target/go-reverse-proxy     -ldflags="-w -s" ./
	GOOS=windows GOARCH=amd64 go build -o target/go-reverse-proxy.exe -ldflags="-w -s" ./
	ls -lh target/

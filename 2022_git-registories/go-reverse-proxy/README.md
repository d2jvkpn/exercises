# go-reverse-proxy
ReverseProxy in golang

*a clone of https://github.com/ilanyu/ReverseProxy*

## Use:

	./target/go-reverse-proxy -h
	
	Usage of go-reverse-proxy:
	  -bind string
	        listen on ip:port (default ":8080")
	  -remote string
	        reverse proxy addr (default "http://example.com")


	./go-reverse-proxy -bind ":8080" -remote https://example.com --addr 127.0.0.1:4242

	Listening on :8080, forwarding to https://example.com, 127.0.0.1:4242

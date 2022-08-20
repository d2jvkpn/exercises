package main

import (
	"log"

	"proglog/internal/server"
)

func main() {
	addr := ":8080"

	log.Printf(">>> http server is listening on %q\n", addr)
	srv := server.NewHTTPServer(addr)
	log.Fatal(srv.ListenAndServe())
}

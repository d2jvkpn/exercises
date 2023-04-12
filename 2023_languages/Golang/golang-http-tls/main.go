package main

import (
	"context"
	"crypto/tls"
	"fmt"
	"log"
	"net"
	"net/http"
	"time"
)

func main() {
	serve02()
}

func serve01() {
	var (
		err    error
		server *http.Server
	)

	server = &http.Server{
		Addr: ":8443",
	}

	// Define a handler function that returns "Hello, world!"
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, world!")
	})

	// Start the HTTPS server
	log.Println("Listening on https://localhost:8443")
	err = server.ListenAndServeTLS("configs/server.crt", "configs/server.key")
	if err != nil {
		log.Fatalf("Failed to start server: %v\n", err)
	}
}

func serve02() {
	var (
		addr     string
		err      error
		cert     tls.Certificate
		listener net.Listener
		mux      *http.ServeMux
		server   *http.Server
	)

	addr = "0.0.0.0:8443"

	// Load the self-signed SSL certificate and key created by OpenSSL
	cert, err = tls.LoadX509KeyPair("configs/server.crt", "configs/server.key")
	if err != nil {
		log.Fatalf("Failed to load certificate: %v\n", err)
	}

	if listener, err = net.Listen("tcp", addr); err != nil {
		log.Fatalf("Failed to listen %q: %v\n", addr, err)
	}

	/*
		// Define a handler function that returns "Hello, world!"
		http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
			fmt.Fprintf(w, "Hello, world!")
		})
	*/

	mux = http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, world!")
	})

	// Create a new HTTPS server using the self-signed certificate
	server = &http.Server{
		// Addr:      addr,
		ReadTimeout:       10 * time.Second,
		WriteTimeout:      10 * time.Second,
		ReadHeaderTimeout: 2 * time.Second,
		MaxHeaderBytes:    2 << 11, // 4K
		// IdleTimeout: ??,
		// MaxMultipartMemory: 8<<20,
		TLSConfig: &tls.Config{Certificates: []tls.Certificate{cert}},
		Handler:   mux,
	}

	// Start the HTTPS server
	log.Printf("Listening on https://%s\n", addr)
	// server.ListenAndServeTLS("", "")
	if err = server.ServeTLS(listener, "", ""); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}

func shutdown(server *http.Server) {
	ctx, cancel := context.WithTimeout(context.TODO(), 5*time.Second)
	if err := server.Shutdown(ctx); err != nil {
		log.Printf("server shutdown: %v\n", err)
	}

	cancel()
}

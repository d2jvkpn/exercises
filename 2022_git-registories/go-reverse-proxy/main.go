package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/d2jvkpn/go-web/pkg/misc"
)

func init() {
	misc.RegisterLogPrinter()
}

func main() {
	var (
		bind   string
		remote string
		addr   string
		err    error

		quit    chan os.Signal
		errch   chan error
		handler *Handler
		server  *http.Server
	)

	flag.StringVar(&bind, "bind", "0.0.0.0:8080", "listen on ip:port")

	flag.StringVar(
		&remote, "remote", "",
		"reverse proxy url: http://example.com, https://example.com",
	)

	flag.StringVar(&addr, "addr", "", "reverse proxy addr: {IP}:{Port}")

	flag.Parse()

	if remote == "" {
		log.Fatalln("remote is unset")
	}

	if handler, err = NewHandler(remote, addr); err != nil {
		log.Fatalln(err)
	}

	server = &http.Server{
		Addr:    bind,
		Handler: handler,
	}

	log.Printf("Listening on %s, forwarding to %s(%s)\n", bind, handler.ReverseProxy, handler.Addr)
	errch, quit = make(chan error, 1), make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt, syscall.SIGTERM)

	shutdown := func() {
		var err error

		ctx, cancel := context.WithTimeout(context.TODO(), 5*time.Second)
		if err = server.Shutdown(ctx); err != nil {
			log.Printf("server shutdown: %v\n", err)
		}
		cancel()
	}

	go func() {
		var err error

		if err = server.ListenAndServe(); err == http.ErrServerClosed {
			err = nil
		} else {
			shutdown()
		}
		errch <- err
	}()

	select {
	case err = <-errch:
	case <-quit:
		fmt.Println("")
		shutdown()
		err = <-errch
	}

	if err != nil {
		log.Fatalln(err)
	}
}

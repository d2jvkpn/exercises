package proxy

import (
	"context"
	"fmt"
	// "log"
	"net"

	socks5 "github.com/armon/go-socks5"
)

// network, address
type Dial = func(string, string) (net.Conn, error)

func Socks5Proxy(dial Dial, localAddr string, auths ...BasicAuth) (
	listener net.Listener, err error) {
	var (
		config      *socks5.Config
		server      *socks5.Server
		authMethods []socks5.Authenticator
	)

	if len(auths) > 0 {
		authMethods = append(authMethods, socks5.UserPassAuthenticator{Credentials: &auths[0]})
	}

	if listener, err = net.Listen("tcp", localAddr); err != nil {
		return nil, err
	}

	config = &socks5.Config{
		AuthMethods: authMethods,
		// Resolver: &socks5.DNSResolver{}, // NameResolver, TODO: filter domains
		Dial: func(ctx context.Context, network, address string) (conn net.Conn, err error) {
			// log.Println("<--", network, address) // tcp, 1.2.3.4:443
			if conn, err = dial(network, address); err != nil {
				return nil, fmt.Errorf("dial %s://%s: %w", network, address, err)
			}
			return conn, nil
		},
	}

	if server, err = socks5.New(config); err != nil {
		return nil, err
	}

	// err = server.ListenAndServe("tcp", localAddr)
	go func() {
		_ = server.Serve(listener)
		// get an error when listener.Close():
		// accept tcp 127.0.0.1:8081: use of closed network connection
	}()

	return listener, nil
}

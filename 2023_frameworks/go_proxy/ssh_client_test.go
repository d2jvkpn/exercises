package proxy

import (
	"fmt"
	"net"
	"testing"
	"time"

	socks5 "github.com/armon/go-socks5"
	. "github.com/stretchr/testify/require"
)

// $ go test -run TestSshClient_t1 -timeout 10m
func TestSshClient_t1(t *testing.T) {
	client, err := NewSshClient("../../configs/ssh.yaml", "ssh")
	NoError(t, err)

	output, err := client.RunCommand("ls")
	NoError(t, err)
	fmt.Printf("~~~ output: %s\n", output)

	listener, err := client.Forward("localhost:1091", "localhost:8000")
	NoError(t, err)

	time.Sleep(5 * time.Minute)
	listener.Close()
}

// $ go test -run TestSocks5Proxy -timeout 10m
func TestSocks5Proxy(t *testing.T) {
	client, err := NewSshClient("../../configs/ssh.yaml", "ssh")
	NoError(t, err)

	cli, err := client.Dial()
	NoError(t, err)
	defer cli.Close()

	listener, err := Socks5Proxy(cli.Dial, "localhost:8080")
	NoError(t, err)

	time.Sleep(5 * time.Minute)
	_ = listener.Close()
}

// $ go test -run TestSshSocks5Proxy -timeout 10m
func TestSshSocks5Proxy(t *testing.T) {
	client, err := NewSshClient("../../configs/ssh.yaml", "ssh")
	NoError(t, err)

	cli, err := client.Dial()
	NoError(t, err)
	defer cli.Close()

	shutdown, err := client.Socks5Proxy("localhost:8080", 15)
	NoError(t, err)

	time.Sleep(5 * time.Minute)
	shutdown()
}

// $ go test -run TestSshDialThrough
func TestSshDialThrough(t *testing.T) {
	client1, err := NewSshClient("../../configs/ssh.yaml", "ssh")
	NoError(t, err)

	client2, err := NewSshClient("../../configs/ssh.yaml", "ssh2")
	NoError(t, err)

	cli, shutdown, err := client2.DialThrough(client1)
	NoError(t, err)
	defer shutdown()

	sess, err := cli.NewSession()
	NoError(t, err)
	defer sess.Close()

	output, err := sess.CombinedOutput("ls")
	NoError(t, err)
	fmt.Printf("~~~ output: %s\n", output)
}

// $ go test -run TestSocks5 -timeout 1m
func TestSocks5(t *testing.T) {
	var (
		err      error
		listener net.Listener
		server   *socks5.Server
	)

	config := &socks5.Config{}
	server, err = socks5.New(config)
	NoError(t, err)

	listener, err = net.Listen("tcp", "localhost:8081")
	NoError(t, err)

	go func() {
		err := server.Serve(listener)
		if err != nil {
			fmt.Println("!!! ServeConn:", err)
		}
	}()

	time.Sleep(30 * time.Second)
	err = listener.Close()
	NoError(t, err)
}

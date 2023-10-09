package proxy

import (
	"crypto/x509"
	"encoding/pem"
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	// "log"
	"net"
	"os"
	"sync"
	"time"

	"github.com/d2jvkpn/gotk"

	"github.com/spf13/viper"
	"golang.org/x/crypto/ssh"
)

type SshConfig struct {
	User     string `mapstructure:"user"`
	Password string `mapstructure:"password"`

	PrivateKeyPath     string `mapstructure:"private_key_path"`
	PrivateKeyPassword string `mapstructure:"private_key_password"`

	Addr        string `mapstructure:"Addr"`
	Port        string `mapstructure:"port"`
	TimeoutSecs int64  `mapstructure:"timeout_secs"`
}

type SshClient struct {
	config SshConfig
	cc     *ssh.ClientConfig
	logger gotk.LogIntf
}

func (config *SshConfig) Address() string {
	return fmt.Sprintf("%s:%s", config.Addr, config.Port)
}

func NewSshClient(fp, key string) (client *SshClient, err error) {
	//
	vp := viper.New()
	vp.SetConfigType("yaml")

	vp.SetConfigFile(fp)
	if err = vp.ReadInConfig(); err != nil {
		return nil, fmt.Errorf("ReadInConfig(): %q, %w", fp, err)
	}

	config := SshConfig{}
	if err = vp.UnmarshalKey(key, &config); err != nil {
		return nil, err
	}

	if config.Password != "" && config.PrivateKeyPath != "" {
		return nil, fmt.Errorf("both password and private_key_path are empty")
	}

	//
	client = &SshClient{config: config, logger: gotk.NewDefaultLogger(os.Stdout, true)}
	cc := &ssh.ClientConfig{
		User: config.User,
		// Auth: []ssh.AuthMethod{},
		// Timeout: 30 * time.Second,
		// ssh.InsecureIgnoreHostKey()
		HostKeyCallback: func(hostname string, remote net.Addr, key ssh.PublicKey) error {
			return nil
		},
	}
	client.cc = cc

	if config.TimeoutSecs > 0 {
		cc.Timeout = time.Duration(config.TimeoutSecs) * time.Second
	}

	if config.Password != "" {
		cc.Auth = []ssh.AuthMethod{ssh.Password(config.Password)}
	} else {
		var signer ssh.Signer
		signer, err = SshSignerFromPem(config.PrivateKeyPath, config.PrivateKeyPassword)
		if err != nil {
			return nil, err
		}
		cc.Auth = []ssh.AuthMethod{ssh.PublicKeys(signer)}
	}

	return client, nil
}

func (client *SshClient) WithLogger(logger gotk.LogIntf) *SshClient {
	client.logger = logger
	return client
}

func SshSignerFromPem(fp, password string) (signer ssh.Signer, err error) {
	var (
		pemBytes []byte
		key      any
		pemBlock *pem.Block
	)

	if pemBytes, err = ioutil.ReadFile(fp); err != nil {
		return nil, err
	}

	if pemBlock, _ = pem.Decode(pemBytes); pemBlock == nil {
		return nil, errors.New("Pem decode failed, no key found")
	}

	if !x509.IsEncryptedPEMBlock(pemBlock) {
		if signer, err = ssh.ParsePrivateKey(pemBytes); err != nil {
			return nil, fmt.Errorf("Parsing plain private key failed: %w", err)
		}
		return signer, nil
	}

	if pemBlock.Bytes, err = x509.DecryptPEMBlock(pemBlock, []byte(password)); err != nil {
		return nil, fmt.Errorf("Decrypting PEM block failed: %w", err)
	}

	switch pemBlock.Type {
	case "RSA PRIVATE KEY":
		key, err = x509.ParsePKCS1PrivateKey(pemBlock.Bytes)
	case "EC PRIVATE KEY":
		key, err = x509.ParseECPrivateKey(pemBlock.Bytes)
	case "DSA PRIVATE KEY":
		key, err = ssh.ParseDSAPrivateKey(pemBlock.Bytes)
	default:
		return nil, fmt.Errorf("Unsupported key type %q", pemBlock.Type)
	}

	if err != nil {
		return nil, fmt.Errorf("Parsing {%v} private key failed: %w", pemBlock.Type, err)
	}

	if signer, err = ssh.NewSignerFromKey(key); err != nil {
		return nil, fmt.Errorf("Creating signer from encrypted key failed: %w", err)
	}
	return signer, nil
}

func (client *SshClient) Dial() (cli *ssh.Client, err error) {
	return ssh.Dial("tcp", client.config.Address(), client.cc)
}

func (client *SshClient) RunCommand(cmd string) (bts []byte, err error) {
	var (
		cli  *ssh.Client
		sess *ssh.Session
	)

	if cli, err = client.Dial(); err != nil {
		return nil, fmt.Errorf("Dial failed: %w", err)
	}
	defer cli.Close()

	if sess, err = cli.NewSession(); err != nil {
		return nil, fmt.Errorf("Create session failed: %w", err)
	}
	defer sess.Close()

	return sess.CombinedOutput(cmd)
}

func (client *SshClient) Forward(localAddr, remoteAddr string) (listener net.Listener, err error) {
	if listener, err = net.Listen("tcp", localAddr); err != nil {
		return nil, err
	}

	tag := fmt.Sprintf("Forward(%s)", remoteAddr)

	forward := func(localConn net.Conn) (cli *ssh.Client, err error) {
		var (
			wg   *sync.WaitGroup
			conn net.Conn
		)

		client.logger.Info("%s, start\n", tag)

		wg = new(sync.WaitGroup)
		if cli, err = client.Dial(); err != nil {
			client.logger.Error("%s, ssh.Dial failed: %v\n", tag, err)
			return nil, err
		}
		defer cli.Close()

		if conn, err = cli.Dial("tcp", remoteAddr); err != nil {
			client.logger.Error("%s, ssh.Client.Dial failed: %v\n", tag, err)
			return nil, err
		}

		wg.Add(1)
		go func() {
			_, err := io.Copy(conn, localConn)
			if err != nil && err != io.EOF {
				client.logger.Error("%s, io.Copy write failed: %v\n", tag, err)
			}
			wg.Done()
		}()

		wg.Add(1)
		go func() {
			_, err := io.Copy(localConn, conn)
			if err != nil && err != io.EOF {
				client.logger.Error("%s, io.Copy read failed: %v\n", tag, err)
			}
			wg.Done()
		}()

		wg.Wait()
		client.logger.Warn("%s, exit\n", tag)
		return cli, nil
	}

	go func() {
		for {
			localConn, err := listener.Accept()
			if err != nil {
				// client.logger.Error("localListener.Accept failed: %v\n", err)
				continue
			}
			go forward(localConn)
		}
	}()

	return listener, nil
}

// https://stackoverflow.com/questions/35906991/go-x-crypto-ssh-how-to-establish-ssh-connection-to-private-instance-over-a-ba
func (client *SshClient) DialThrough(bastion *SshClient) (cli *ssh.Client, shutdown func(), err error) {
	var (
		bCli  *ssh.Client
		conn1 net.Conn
		conn2 ssh.Conn
		nch   <-chan ssh.NewChannel
		rch   <-chan *ssh.Request
	)

	if bCli, err = bastion.Dial(); err != nil {
		return nil, nil, err
	}

	if conn1, err = bCli.Dial("tcp", client.config.Address()); err != nil {
		return nil, nil, err
	}

	conn2, nch, rch, err = ssh.NewClientConn(conn1, client.config.Address(), client.cc)
	if err != nil {
		return nil, nil, err
	}

	cli = ssh.NewClient(conn2, nch, rch)
	shutdown = func() {
		cli.Close()
		bCli.Close()
	}

	return cli, shutdown, nil
}

func (client *SshClient) Socks5Proxy(localAddr string, secs int64, auths ...BasicAuth) (
	shutdown func(), err error) {
	var (
		closed   bool
		ticker   *time.Ticker
		mutex    *sync.RWMutex
		cli      *ssh.Client
		listener net.Listener
	)

	if cli, err = client.Dial(); err != nil {
		return nil, err
	}

	mutex = new(sync.RWMutex)
	closed = false

	dial := func(network, address string) (conn net.Conn, err error) {
		mutex.RLock()
		defer mutex.RUnlock()

		if closed {
			return nil, fmt.Errorf("connection with ssh.Client was closed")
		}

		conn, err = cli.Dial(network, address)
		if err == nil {
			// client.logger.Debug("dial %s is ok\n", address)
		} else {
			client.logger.Warn("dial %s: %v\n", address, err)
		}
		return conn, err
	}

	if listener, err = Socks5Proxy(dial, localAddr, auths...); err != nil {
		return nil, err
	}

	go func() {
		if secs <= 0 {
			return
		}

		var (
			err  error
			sess *ssh.Session
		)

		ticker = time.NewTicker(time.Duration(secs) * time.Second)

		for _ = range ticker.C {
			if sess, err = cli.NewSession(); err == nil {
				// client.logger.Debug("connection with ssh.Client is ok")
				sess.Close()
				continue
			}

			mutex.Lock()
			// client.logger.Debug("try to reconnect to ssh.Client\n")
			if cli, err = client.Dial(); err != nil {
				client.logger.Warn("client.Dial() failed: %v\n", err)
			} else {
				// client.logger.Debug("reconnected to ssh.Client\n")
			}
			mutex.Unlock()
		}

		// client.logger.Debug("ticker was closed\n")
	}()

	shutdown = func() {
		mutex.Lock()
		defer mutex.Unlock()

		if closed {
			return
		}
		closed = true
		if ticker != nil {
			ticker.Stop()
		}

		_ = listener.Close()
		_ = cli.Close()
	}

	return shutdown, nil
}

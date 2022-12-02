package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"math/rand"
	"net"
	"net/http"
	"net/http/httputil"
	"net/url"
	"strings"
	"time"

	"github.com/miekg/dns"
)

type Handler struct {
	Addr         string
	servers      []string
	retryTimes   int
	rand         *rand.Rand
	ReverseProxy *url.URL
	dialer       *net.Dialer
}

func NewHandler(remote, addr string, servers ...string) (handler *Handler, err error) {
	handler = &Handler{
		Addr:       addr,
		retryTimes: 5,
		rand:       rand.New(rand.NewSource(time.Now().UnixNano())),
	}

	if len(servers) == 0 {
		servers = []string{
			"114.114.114.114:53", "114.114.115.115:53", "119.29.29.29:53", "223.5.5.5:53",
			"8.8.8.8:53", "208.67.222.222:53", "208.67.220.220:53",
		}
	}
	handler.servers = servers

	handler.dialer = &net.Dialer{
		Timeout:   30 * time.Second,
		KeepAlive: 30 * time.Second,
		DualStack: true,
	}

	if handler.ReverseProxy, err = url.Parse(remote); err != nil {
		return nil, err
	}

	return handler, nil
}

func (handler *Handler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	log.Printf(
		"remote_addr: %s, method: %s, url: %s, proto: %s, user-agent: %q\n",
		r.RemoteAddr, r.Method, r.URL.String(), r.Proto, r.UserAgent(),
	)

	http.DefaultTransport.(*http.Transport).DialContext = func(
		ctx context.Context, network, addr string) (net.Conn, error) {
		var (
			host, port string
			err        error
			ipList     []net.IP
		)

		// process r: http.Request
		if handler.Addr != "" {
			return handler.dialer.DialContext(ctx, network, handler.Addr)
		}

		host, port, _ = net.SplitHostPort(addr)
		if ipList, err = handler.LookupHost(host); err != nil {
			return nil, err
		}

		return handler.dialer.DialContext(
			ctx, network,
			fmt.Sprintf("%s:%s", ipList[0].String(), port),
		)
	}

	proxy := httputil.NewSingleHostReverseProxy(handler.ReverseProxy)
	r.Host = handler.ReverseProxy.Host
	proxy.ServeHTTP(w, r)
}

func (handler *Handler) LookupHost(host string) (ipList []net.IP, err error) {
	var msg, ans *dns.Msg

	msg = &dns.Msg{
		MsgHdr: dns.MsgHdr{Id: dns.Id(), RecursionDesired: true},
	}
	msg.Question = []dns.Question{
		{Name: dns.Fqdn(host), Qtype: dns.TypeA, Qclass: dns.ClassINET},
	}

	server := handler.servers[handler.rand.Intn(len(handler.servers))]

	for i := 0; i < handler.retryTimes; i++ {
		if ans, err = dns.Exchange(msg, server); err == nil {
			break
		} else if !strings.HasSuffix(err.Error(), "i/o timeout") {
			return nil, err
		}
	}
	if err != nil {
		return nil, err
	}

	if ans.Rcode != dns.RcodeSuccess {
		return nil, errors.New(dns.RcodeToString[ans.Rcode])
	}

	for _, v := range ans.Answer {
		if e, ok := v.(*dns.A); ok {
			ipList = append(ipList, e.A)
		}
	}
	if len(ipList) == 0 {
		return nil, fmt.Errorf("failed to query ip")
	}
	return ipList, nil
}

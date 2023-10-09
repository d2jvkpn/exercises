// Copyright 2015 The Gorilla WebSocket Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//go:build ignore

package main

import (
	"bytes"
	"flag"
	"log"
	"net/url"
	"os"
	"os/signal"
	"time"

	"github.com/d2jvkpn/pieces/pkg/go/misc"
	"github.com/gorilla/websocket"
)

const RFC3339ms = "2006-01-02T15:04:05.000Z07:00"

func init() {
	misc.SetLogRFC3339()
}

func main() {
	var (
		err  error
		link *url.URL
		conn *websocket.Conn
	)

	addr := flag.String(
		"addr",
		"ws://localhost:8080/ws/open/talk?token=D8N7ioBuiufUX8km0Ert8i5vhj2k9XFV",
		"websocket service address",
	)
	pingPerSecs := flag.Uint64("pingPerSecs", 5, "send ping per seconds")
	flag.Parse()

	if *pingPerSecs == 0 {
		log.Fatal("invalid pingPerSecs")
	}

	// link := url.URL{Scheme: "ws", Host: *addr, Path: "/ws/talk"}
	if link, err = url.Parse(*addr); err != nil {
		log.Fatal("parse url:", err)
	}
	log.Printf("connecting to %s\n", link.String())

	if conn, _, err = websocket.DefaultDialer.Dial(link.String(), nil); err != nil {
		log.Fatal("dial:", err)
	}
	defer conn.Close()

	interrupt := make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt)
	done := make(chan struct{}, 2)
	ticker := time.NewTicker(time.Duration(*pingPerSecs) * time.Second)
	defer ticker.Stop()

	conn.SetPongHandler(func(appData string) (err error) {
		log.Println("pong: ~")
		return nil
	})

	conn.SetCloseHandler(func(code int, text string) error {
		log.Println("server closed connection: code=%d, text=%q", code, text)
		done <- struct{}{}
		return nil
	})

	go HandleMessage(conn, done)

	writeKindMessage := func(kind string, mp map[string]interface{}) error {
		if mp == nil {
			mp = make(map[string]interface{}, 1)
		}
		mp["kind"] = kind
		return conn.WriteJSON(mp) // websocket.TextMessage
	}

	writeKindMessage("hello", map[string]interface{}{
		"at": time.Now().UnixMilli(), "name": "Rover", "message": "My name is Rover.",
	})

loop:
	for {
		select {
		case <-done:
			break loop
		case <-interrupt:
			log.Println("interrupted")
			// Cleanly close the connection by sending a close message and then
			// waiting (with timeout) for the server to close the connection.
			if err = conn.WriteMessage(
				websocket.CloseMessage,
				websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""),
			); err != nil {
				log.Println("write close:", err)
				break loop
			}

			select {
			case <-done:
			case <-time.After(time.Second):
			}
			break loop
		case <-ticker.C: // t := <-ticker.C;  t.String()
			// err = conn.WriteMessage(websocket.TextMessage, []byte(t.Format(RFC3339ms)))
			log.Println("ping: ~")
			if err = conn.WriteMessage(websocket.PingMessage, nil); err != nil {
				log.Printf("WriteMesaage-Ping error: %[1]T, %[1]v\n", err)
				ticker.Stop()
				break loop
			}
		}
	}
}

func HandleMessage(conn *websocket.Conn, done chan struct{}) {
	var (
		bts []byte
		err error
	)

	for {
		if _, bts, err = conn.ReadMessage(); err != nil {
			log.Printf("ReadMessage: %[1]T, %[1]v\n", err)
			return
		}
		log.Printf("recv: %s\n", bytes.TrimSpace(bts))
	}

	done <- struct{}{}
}

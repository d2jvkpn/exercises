package internal

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	// "net"
	// "net/http"
	"runtime"
	"sync/atomic"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
)

var (
	_ClientId uint64 = 0

	ClientsMap = make(map[string]*Client, 100)
)

type Client struct {
	Id       string    `json:"id"`
	Address  string    `json:"address"`
	Online   bool      `json:"online"`
	CreatAt  time.Time `json:"createdAt"`
	ClosedAt time.Time `json:"closedAt"`
	LastPing time.Time `json:"lastPing"`
}

func newClientId() string {
	return fmt.Sprintf("client%06d", atomic.AddUint64(&_ClientId, 1))
}

func (client Client) String() string {
	bts, _ := json.Marshal(client)
	return string(bts)
}

func LoadWebsocket(rg *gin.RouterGroup, handlers ...gin.HandlerFunc) {
	ws := rg.Group("/ws/open", handlers...)
	ws.GET("/talk", talk)
}

func talk(ctx *gin.Context) {
	var (
		addr, token string
		clientId    string
		err         error
		quit        chan struct{}
		conn        *websocket.Conn
		client      Client
	)

	addr = ctx.ClientIP()
	if conn, err = _Upgrader.Upgrade(ctx.Writer, ctx.Request, nil); err != nil {
		log.Printf("%s talk upgrade error: %v\n", addr, err)
		return
	}
	defer conn.Close()

	clientId = newClientId()
	now := time.Now()
	client = Client{Id: clientId, Address: addr, Online: true, CreatAt: now, LastPing: now}

	token = ctx.DefaultQuery("token", "")
	ClientsMap[clientId] = &client

	// to avoid dead lock when for loop blocked by HandleMessage, don't use an unbuffered channel
	quit = make(chan struct{}, 1)
	log.Printf("%s connected: addr=%q, token=%q\n", clientId, addr, token)

	conn.SetPingHandler(func(appData string) (err error) {
		log.Printf("%s ping: ~\n", clientId)
		client.LastPing = time.Now()
		conn.WriteMessage(websocket.PongMessage, nil)
		return nil
	})

	conn.SetCloseHandler(func(code int, text string) error {
		log.Printf("%s closed: code=%d, text=%q\n", clientId, code, text)
		// println("~~~ before send to quit")
		quit <- struct{}{}
		// println("~~~ after send to quit")
		return nil
	})

	go func() {
		// client sends ping every 5 seconds
		var dur = 5 * time.Second
		var ticker = time.NewTicker(time.Duration(2) * dur)
		for {
			select {
			case now := <-ticker.C:
				if !client.Online {
					ticker.Stop()
					return
				}
				delta := now.Sub(client.LastPing)
				lastPing := client.LastPing.Format(TimeFormat)
				if delta > dur {
					log.Printf("%s lastPing: %v, delat: %v", clientId, lastPing, delta)
				}
			}
		}
	}()

	log.Printf("Num Goroutine: %d\n", runtime.NumGoroutine())

loop:
	for {
		// println("~~~ loop")
		select {
		case <-quit:
			break loop
		default:
			// this can block the loop
			if err = HandleMessage(conn, clientId); err != nil {
				log.Printf("%s HandleMessage error: %v\n", clientId, err)
				switch err.(type) {
				// close 1006 (abnormal closure): unexpected EOF
				case *websocket.CloseError:
					break loop
				}
			}
		}
	}

	client.Online, client.ClosedAt = false, time.Now()
	go func() {
		<-time.After(10 * time.Second)
		log.Printf("delete client: %s\n", client)
		delete(ClientsMap, client.Id)
	}()
	// when this routime exit, gin will print a http request log

	log.Printf("Num Goroutine: %d\n", runtime.NumGoroutine())
}

func HandleMessage(conn *websocket.Conn, clientId string) (err error) {
	var (
		ok   bool
		mt   int
		bts  []byte
		kind string
		// addr net.Addr
		data, res map[string]interface{}
	)

	// addr = conn.RemoteAddr()
	if mt, bts, err = conn.ReadMessage(); err != nil {
		return
	}

	defer func() {
		if bts, err = json.Marshal(res); err != nil {
			return
		}
		err = conn.WriteMessage(mt, bts)
	}()

	data = make(map[string]interface{})
	if json.Unmarshal(bts, &data); err != nil {
		res = map[string]interface{}{"kind": "error", "message": "unmarshal message error"}
		return
	}
	log.Printf("%s recv: %s\n", clientId, bytes.TrimSpace(bts))

	if kind, ok = data["kind"].(string); !ok {
		res = map[string]interface{}{"kind": "error", "message": "invalid field kind"}
		return
	}

	if kind == "hello" {
		name, _ := data["name"].(string)
		res = map[string]interface{}{
			"kind": "hello", "id": clientId, "message": fmt.Sprintf("Welcome %s!", name),
		}
		return
	}

	res = map[string]interface{}{"kind": "unknown", "message": "unkonw message kind"}
	return
}

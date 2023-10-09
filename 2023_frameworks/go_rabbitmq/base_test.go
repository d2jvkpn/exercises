package rabbitmq

import (
	"flag"
	"fmt"
	"testing"
)

var (
	testUri = "amqp://guest:guest@localhost:5672"
)

// go test -run TestTopicQueue -- URLURL
func TestMain(m *testing.M) {
	flag.Parse()

	if args := flag.Args(); len(args) > 0 {
		testUri = args[0]
	}

	fmt.Println("~~~ testUri:", testUri)
	m.Run()
}

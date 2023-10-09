package greet

import (
	"encoding/json"
	"fmt"
	"testing"
	"time"

	"google.golang.org/protobuf/proto"
)

// TODO: benchmark funcs
func TestProtoMarshal(t *testing.T) {
	msg := Msg{Code: 101, Text: "hello, world"}

	var (
		bts []byte
		err error
		now time.Time
	)

	// proto
	now = time.Now()
	if bts, err = proto.Marshal(&msg); err != nil {
		t.Fatal(err)
	}
	fmt.Printf(">>> %s\n    %d bytes, %v\n", time.Since(now), len(bts), bts)

	now = time.Now()
	_ = proto.Unmarshal(bts, &msg)
	fmt.Printf("    %s\n", time.Since(now))

	// json
	now = time.Now()
	if bts, err = json.Marshal(&msg); err != nil {
		t.Fatal(err)
	}
	fmt.Printf(">>> %s\n    %d bytes, %v\n", time.Since(now), len(bts), bts)
	now = time.Now()
	_ = json.Unmarshal(bts, &msg)
	fmt.Printf("    %s\n", time.Since(now))
}

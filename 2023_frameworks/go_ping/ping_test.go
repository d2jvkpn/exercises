package ping

import (
	"encoding/json"
	"fmt"
	"testing"
	"time"

	probing "github.com/prometheus-community/pro-bing"
)

func TestPingDev(t *testing.T) {
	pinger, err := probing.NewPinger("www.baidu.com")
	// pinger, err := probing.NewPinger("127.0.0.1")
	// pinger, err := probing.NewPinger("www.google.com")
	if err != nil {
		t.Fatal(err)
	}

	pinger.Count = 100
	fmt.Println(">>> Run")

	ch := make(chan bool, 1)

	go func() {
		err = pinger.Run() // Blocks until finished.
		ch <- err == nil
	}()

	ok := false

	select {
	case <-time.After(3 * time.Second):
		fmt.Println("~~~ timeout")
		pinger.Stop()
		fmt.Println("??? err:", err) // err == nil
		<-ch
		ok = true
	case ok = <-ch:
		fmt.Println("~~~ done")
	}

	if !ok {
		t.Fatal(err)
	}

	fmt.Printf("%+v\n", pinger.Statistics())
}

func TestPingTask(t *testing.T) {
	pt := &PingTask{
		Every:     time.Hour,
		TimeoutMs: 200,
		ConcNum:   64,
	}

	if err := pt.LoadFromJson("../../configs/devices/", "items_"); err != nil {
		t.Fatal(err)
	}

	pt.run()
	bts, err := json.Marshal(pt.Reports())
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("~~~ %s\n", bts)
}

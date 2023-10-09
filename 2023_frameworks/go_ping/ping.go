package ping

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"path/filepath"
	"sync"
	"time"

	probing "github.com/prometheus-community/pro-bing"
)

type PingTask struct {
	Sources   []string      `mapstructure:"-"`
	Every     time.Duration `mapstructure:"every"`
	TimeoutMs int64         `mapstructure:"timeout_ms"`
	ConcNum   int           `mapstructure:"conc_num"`

	bts     []byte
	mutex   *sync.RWMutex
	ticker  *time.Ticker
	results []Result
	reports []Summary
}

type Result struct {
	Begin   time.Time `json:"begin"`
	End     time.Time `json:"end"`
	Kind    string    `json:"kind"`
	Devices []Device  `json:"devices"`
}

type Device struct {
	Ip         string              `json:"ip"`
	Name       string              `json:"name,omitempty"`
	Status     string              `json:"status,omitempty"` // online, offline
	Error      string              `json:"error,omitempty"`
	Statistics *probing.Statistics `json:"statistics,omitempty"`
}

type Summary struct {
	Begin   time.Time `json:"begin"`
	End     time.Time `json:"end"`
	Kind    string    `json:"kind"`
	Online  int       `json:"online"`
	Offline int       `json:"offline"`
}

func (pt *PingTask) Validate() (err error) {
	if pt.Every <= 0 || pt.TimeoutMs <= 0 {
		return fmt.Errorf("invalid every or timeout_ms")
	}

	if pt.ConcNum <= 0 {
		return fmt.Errorf("invalid conc_num")
	}

	return nil
}

func (pt *PingTask) LoadFromJson(dir, prefix string) (err error) {
	var bts []byte

	pt.Sources, err = filepath.Glob(filepath.Join(dir, prefix+"*.json"))
	if err != nil {
		return err
	}
	if len(pt.Sources) == 0 {
		return fmt.Errorf("no match source files")
	}

	pt.results = make([]Result, 0, len(pt.Sources))
	for _, v := range pt.Sources {
		if bts, err = ioutil.ReadFile(v); err != nil {
			return err
		}

		devices := make([]Device, 0)
		if err = json.Unmarshal(bts, &devices); err != nil {
			return fmt.Errorf("unmarshal %s: %w", v, err)
		}

		for i := range devices {
			devices[i].Status = ""
			devices[i].Error = ""
			devices[i].Statistics = nil
		}

		kind := []rune(filepath.Base(v))
		l := len(kind)
		// fmt.Println("~~~ kind:", filepath.Base(v), kind, len([]rune(prefix)), l - 5)
		pt.results = append(pt.results, Result{
			// Begin: ,
			// End: ,
			Kind:    string(kind[len([]rune(prefix)):(l - 5)]),
			Devices: devices,
		})
	}

	pt.mutex = new(sync.RWMutex)
	pt.ticker = time.NewTicker(pt.Every)
	return nil
}

func (pt *PingTask) Run() {
	go pt.run()

	go func() {
		for _ = range pt.ticker.C {
			pt.run()
		}
	}()
}

func (pt *PingTask) Stop() {
	pt.ticker.Stop()
}

func (pt *PingTask) run() {
	ping := func(device *Device) {
		pinger, err := probing.NewPinger(device.Ip)
		if err != nil {
			device.Status = "offline"
			device.Error = err.Error()
			return
		}

		pinger.Count = 2
		pinger.Timeout = time.Duration(pt.TimeoutMs) * time.Millisecond

		// Blocks until finished.
		err = pinger.Run()
		device.Statistics = pinger.Statistics()

		switch {
		case err != nil:
			device.Status, device.Error = "offline", err.Error()
		case device.Statistics != nil && device.Statistics.PacketsRecv > 0:
			device.Status, device.Error = "online", ""
		default:
			device.Status, device.Error = "offline", ""
		}
	}

	summary := func(result *Result) (summ Summary) {
		summ.Kind = result.Kind

		for i := range result.Devices {
			if result.Devices[i].Status == "online" {
				summ.Online++
			} else {
				summ.Offline++
			}
		}

		return summ
	}

	n := 0
	done := make(chan struct{}, pt.ConcNum)
	reports := make([]Summary, 0, len(pt.results))

	for i := range pt.results {
		result := &pt.results[i]
		result.Begin = time.Now()

		for j := range result.Devices {
			if n++; n > pt.ConcNum {
				<-done
				n--
			}

			go func(j int) {
				ping(&result.Devices[j])
				done <- struct{}{}
			}(j)
		}

		for ; n > 0; n-- {
			<-done
		}

		result.End = time.Now()
		summ := summary(result)
		summ.Begin, summ.End = result.Begin, result.End
		reports = append(reports, summ)
	}

	pt.mutex.Lock()
	pt.reports = reports
	pt.mutex.Unlock()
}

func (pt *PingTask) Reports() []Summary {
	pt.mutex.RLock()
	reports := make([]Summary, len(pt.reports))
	copy(reports, pt.reports)
	pt.mutex.RUnlock()

	return reports
}

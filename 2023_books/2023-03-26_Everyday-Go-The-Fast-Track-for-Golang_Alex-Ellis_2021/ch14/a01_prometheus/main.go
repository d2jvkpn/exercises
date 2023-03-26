package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io/ioutil"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	hashesInflight = prometheus.NewGauge(prometheus.GaugeOpts{
		Subsystem: "http",
		Name:      "hashes_inflight",
		Help:      "total hashes inflight",
	})
)

func main() {
	port := 8080

	// readTimeout := time.Millisecond * 500
	// writeTimeout := time.Millisecond * 500
	readTimeout := time.Second * 10
	writeTimeout := time.Second * 10

	prometheus.MustRegister(hashesInflight)

	httpMetrics := NewHttpMetrics()
	instrumentedHash := InstrumentHandler(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		if r.Body != nil {
			body, _ := ioutil.ReadAll(r.Body)
			fmt.Fprintf(w, "%s", computeSum(body))
		}
	}, httpMetrics)

	mux := http.NewServeMux()

	mux.Handle("/metrics", promhttp.Handler())
	mux.HandleFunc("/hash", customHash)
	mux.HandleFunc("/hash2", instrumentedHash)

	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "No tracking for this request")

	})

	s := &http.Server{
		Addr:           fmt.Sprintf(":%d", port),
		ReadTimeout:    readTimeout,
		WriteTimeout:   writeTimeout,
		MaxHeaderBytes: 1 << 20, // Max header of 1MB
		Handler:        mux,
	}

	fmt.Println(">>> Http server is listening on:", s.Addr)
	s.ListenAndServe()
}

// computeSum generates a SHA256 hash of the body
func computeSum(body []byte) []byte {
	h := sha256.New()
	h.Write(body)
	hashed := hex.EncodeToString(h.Sum(nil))
	time.Sleep(time.Second * 5)
	return []byte(hashed)
}

func customHash(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	if r.Body != nil {
		defer r.Body.Close()
		hashesInflight.Inc()
		defer hashesInflight.Dec()
		body, _ := ioutil.ReadAll(r.Body)
		fmt.Fprintf(w, "%s", computeSum(body))
	}
}

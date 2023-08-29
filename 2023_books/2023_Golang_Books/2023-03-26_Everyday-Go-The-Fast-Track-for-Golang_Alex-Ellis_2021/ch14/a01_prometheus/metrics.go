package main

import (
	"fmt"
	"net/http"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

type HttpMetrics struct {
	RequestsTotal            *prometheus.CounterVec
	RequestDurationHistogram *prometheus.HistogramVec
}

func NewHttpMetrics() HttpMetrics {
	return HttpMetrics{
		RequestsTotal: promauto.NewCounterVec(prometheus.CounterOpts{
			Subsystem: "http",
			Name:      "requests_total",
			Help:      "total HTTP requests processed",
		}, []string{"code", "method"}),

		RequestDurationHistogram: promauto.NewHistogramVec(prometheus.HistogramOpts{
			Subsystem: "http",
			Name:      "request_duration_seconds",
			Help:      "Seconds spent serving HTTP requests.",
			Buckets:   prometheus.DefBuckets,
		}, []string{"code", "method"}),
	}
}

// InstrumentHandler instruments any HTTP handler for the request
// total and request duration metric
func InstrumentHandler(next http.HandlerFunc, hm HttpMetrics) http.HandlerFunc {
	return promhttp.InstrumentHandlerCounter(
		hm.RequestsTotal,
		promhttp.InstrumentHandlerDuration(hm.RequestDurationHistogram, next),
	)
}

func GaugeHandler(next http.HandlerFunc, name string) (
	inflight prometheus.Gauge, handler http.HandlerFunc) {

	inflight = prometheus.NewGauge(prometheus.GaugeOpts{
		Subsystem: "http",
		Name:      fmt.Sprintf("%s_inflight", name),
		Help:      fmt.Sprintf("total %s inflight", name),
	})

	handler = func(w http.ResponseWriter, r *http.Request) {
		inflight.Inc()
		next(w, r)
		inflight.Dec()
	}

	return inflight, handler
}

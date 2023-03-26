package main

import (
	"fmt"
	"strconv"
	"time"
	// "net/http"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	// "github.com/prometheus/client_golang/prometheus/promhttp"
)

// InstrumentHandler instruments any HTTP handler for the request
// total and request duration metric
func InstrumentHandler(name string) gin.HandlerFunc {
	requestsTotal := promauto.NewCounterVec(prometheus.CounterOpts{
		Subsystem: "http",
		Name:      fmt.Sprintf("%s_count_total", name),
		Help:      fmt.Sprintf("%s requests processed", name),
	}, []string{"status", "method"})

	requestDuration := promauto.NewHistogramVec(prometheus.HistogramOpts{
		Subsystem: "http",
		Name:      fmt.Sprintf("%s_duration_seconds", name),
		Help:      fmt.Sprintf("seconds spent serving requests of %s", name),
		Buckets:   prometheus.DefBuckets,
	}, []string{"status", "method", "path"})

	_ = prometheus.Register(requestsTotal)
	_ = prometheus.Register(requestDuration)

	// https://robert-scherbarth.medium.com/measure-request-duration-with-prometheus-and-golang-adc6f4ca05fe
	return func(ctx *gin.Context) {
		req := ctx.Request
		start := time.Now()
		// timer := prometheus.NewTimer(requestDuration.WithLabelValues(req.Method, req.URL.Path))
		ctx.Next()

		status := strconv.Itoa(ctx.Writer.Status())
		requestsTotal.WithLabelValues(status, req.Method).Inc()
		// timer.ObserveDuration()
		duration_ms := float64(time.Since(start).Milliseconds())
		requestDuration.WithLabelValues(status, req.Method, req.URL.Path).Observe(duration_ms)
	}
}

func GaugeHandler(name string) gin.HandlerFunc {
	inflight := prometheus.NewGauge(prometheus.GaugeOpts{
		Subsystem: "http",
		Name:      fmt.Sprintf("%s_inflight", name),
		Help:      fmt.Sprintf("total %s inflight", name),
	})

	_ = prometheus.Register(inflight)

	return func(ctx *gin.Context) {
		inflight.Inc()
		ctx.Next()
		inflight.Dec()
	}
}

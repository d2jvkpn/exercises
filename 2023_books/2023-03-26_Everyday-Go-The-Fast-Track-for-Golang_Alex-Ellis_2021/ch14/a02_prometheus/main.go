package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io/ioutil"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

func main() {
	port := 8080

	// readTimeout := time.Millisecond * 500
	// writeTimeout := time.Millisecond * 500
	readTimeout := time.Second * 10
	writeTimeout := time.Second * 10

	engine := gin.Default()

	router := &engine.RouterGroup
	router.Use(InstrumentHandler("total"))
	router.GET("/metrics", gin.WrapH(promhttp.Handler()))
	router.POST("/hash", ComputeHashSum, GaugeHandler("HashSum"))

	s := &http.Server{
		Addr:           fmt.Sprintf(":%d", port),
		ReadTimeout:    readTimeout,
		WriteTimeout:   writeTimeout,
		MaxHeaderBytes: 1 << 20, // Max header of 1MB
		Handler:        engine,
	}

	fmt.Println(">>> Http server is listening on:", s.Addr)
	s.ListenAndServe()
}

func ComputeHashSum(ctx *gin.Context) {
	bts, _ := ioutil.ReadAll(ctx.Request.Body)

	if len(bts) > 8 {
		ctx.String(http.StatusBadRequest, "too long")
		return
	}

	h := sha256.New()
	h.Write(bts)
	hashed := hex.EncodeToString(h.Sum(nil))
	time.Sleep(time.Second * 5)

	ctx.String(http.StatusOK, hashed)
}

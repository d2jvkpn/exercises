package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"
)

func main() {
	var (
		bts     []byte
		urlPath string
		data    map[string]interface{}
		err     error
		client  *http.Client
		req     *http.Request
		res     *http.Response
	)

	client = &http.Client{
		Transport: &http.Transport{
			MaxIdleConns:    10,
			IdleConnTimeout: 30 * time.Second,
		},
	}

	if urlPath = "http://localhost:8080/api/open/ping"; len(os.Args) > 1 {
		urlPath = os.Args[1]
	}

	if req, err = http.NewRequest("GET", urlPath, nil); err != nil {
		log.Fatalf("http.NewRequest: %v\n", err)
	}

	// req.Header.Add("If-None-Match", `W/"wyzzy"`)
	if res, err = client.Do(req); err != nil {
		log.Fatalf("Client.Do: %v\n", err)
	}
	defer res.Body.Close()

	if res.StatusCode != http.StatusOK {
		log.Fatalf("Response.StatusCode: %d\n", res.StatusCode)
	}

	////
	if res.Header.Get("Content-Type") != "application/json; charset=utf-8" {
		if bts, err = io.ReadAll(res.Body); err != nil {
			log.Fatalf("io.ReadAll(Response.Body): %v\n", err)
		}

		fmt.Printf("%s\n", bts)
		return
	}

	data = make(map[string]interface{}, 3)
	if err = json.NewDecoder(res.Body).Decode(&data); err != nil {
		log.Fatalf("json.Decode: %v\n", err)
	}
	fmt.Printf("%#v\n", data)
}

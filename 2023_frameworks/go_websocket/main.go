package main

import (
	"context"
	_ "embed"
	"flag"
	// "fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"runtime/debug"
	"strings"
	"syscall"
	"time"

	"hello/internal"

	"github.com/d2jvkpn/pieces/pkg/go/misc"
)

//go:generate go fmt ./...
//go:generate go vet ./...
//go:generate echo -e >>> both "go fmt" and "go vet" are ok

//go:generate echo -e "\n>>> tokie statistic result"
//go:generate tokei

//go:generate echo "\n>>> bash go_build.sh"
//go:generate bash go_build.sh

var (
	//go:embed .version
	version string
)

func init() {
	misc.SetLogRFC3339() // "2006-01-02T15:04:05.000Z07:00"

	if os.Getenv("TZ") == "" {
		os.Setenv("TZ", "Asia/Shanghai")
	}

	version = strings.Fields(version)[0]
}

func main() {
	var (
		releaseMode bool
		mode        string
		addr, pprof string
		cer, key    string

		err       error
		buildInfo [][2]string
		quit      chan os.Signal
		ctx       context.Context
		cancel    context.CancelFunc
		srv       *http.Server
		pp        *misc.Pprof
	)

	flag.BoolVar(&releaseMode, "release", false, "use release mode")
	flag.StringVar(&addr, "addr", ":8080", "service listen address")
	flag.StringVar(
		&pprof, "pprof", "",
		"pprof listen address, default empty string means not enable",
	) // ":1030"
	flag.StringVar(&cer, "cer", "", "certFile path for https")
	flag.StringVar(&key, "key", "", "keyFile path for https")
	flag.Parse()
	// prog := filepath.Base(os.Args[0])
	buildInfo = BuildInfo([2]string{"version", version})

	// https://chenyitian.gitbooks.io/gin-web-framework/docs/38.html
	quit = make(chan os.Signal, 2)
	signal.Notify(quit, os.Interrupt, syscall.SIGTERM, syscall.SIGUSR2)

	if srv, err = internal.NewServer(releaseMode, buildInfo); err != nil {
		log.Fatalln(err)
	}
	srv.Addr = addr

	go func() {
		var e error

		log.Printf(">>> HTTP server listen on %q\n", addr)
		if cer == "" {
			e = srv.ListenAndServe()
		} else {
			e = srv.ListenAndServeTLS(cer, key)
		}
		if e == http.ErrServerClosed {
			log.Printf("HTTP server closed: %v\n", e)
		} else {
			log.Printf("HTTP server error: %v\n", e)
		}

		quit <- syscall.SIGUSR2
	}()

	if pprof != "" {
		pp = misc.NewPprof(pprof)
		defer pp.Shutdown()
		go func() {
			log.Printf(">>> Pprof server run in %q mode, listen on %q\n", mode, pprof)
			if e := pp.Run(); e == http.ErrServerClosed {
				log.Printf("Pprof server closed: %v\n", e)
			} else {
				log.Printf("Pprof server error: %v\n", e)
			}

			quit <- syscall.SIGUSR2
		}()
	}

	<-quit
	signal.Stop(quit)

	ctx, cancel = context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err = srv.Shutdown(ctx); err != nil {
		log.Printf("server shutdown: %v\n", err)
	}
}

func BuildInfo(vars ...[2]string) (info [][2]string) {
	info = make([][2]string, 0, 10)
	buildInfo, _ := debug.ReadBuildInfo()

	info = append(info, [2]string{"goVersion", buildInfo.GoVersion})
	info = append(info, [2]string{"startTime", time.Now().Format(time.RFC3339)})

	parseFlags := func(str string) {
		for _, v := range strings.Fields(str) {
			k, v, _ := strings.Cut(v, "=")
			if strings.HasPrefix(k, "main.") && v != "" {
				info = append(info, [2]string{k[5:], v})
			}
		}
	}

	for _, v := range buildInfo.Settings {
		if v.Key == "-ldflags" || v.Key == "--ldflags" {
			parseFlags(v.Value)
		}
	}

	for i := range vars {
		info = append(info, [2]string{vars[i][0], vars[i][1]})
	}

	return info
}

package main

import (
	_ "embed"
	"flag"
	"fmt"
	"log"

	"golang_postgres-redis/internal"
	"golang_postgres-redis/internal/settings"
)

var (
	//go:embed project.yaml
	_Project string
)

func main() {
	var (
		release bool
		addr    string
		config  string
		err     error
	)

	if err = settings.SetProject(_Project); err != nil {
		log.Fatalln(err)
	}

	flag.StringVar(&addr, "addr", "0.0.0.0:3081", "http server listening address")
	flag.StringVar(&config, "config", "configs/local.yaml", "config file path")
	flag.BoolVar(&release, "release", false, "run in release mode")

	flag.Usage = func() {
		output := flag.CommandLine.Output()

		fmt.Fprintf(output, "Usage:\n")
		flag.PrintDefaults()
		fmt.Fprintf(output, "\nyaml config:\n```yaml\n%s```\n", settings.Config())
		fmt.Fprintf(output, "\n.env:\n```text\n%s\n```\n", settings.DotEnv())
	}

	flag.Parse()

	if err = internal.Load(config, release); err != nil {
		log.Fatalln(err)
	}

	log.Println("ok and exit")
}

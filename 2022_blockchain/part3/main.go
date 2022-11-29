// https://jeiwan.net/posts/building-blockchain-in-go-part-2/
package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"strings"
)

var (
	_TargetBits int = 20
)

func main() {
	var (
		count int
		err   error
		bc    *Blockchain
	)

	flag.IntVar(&_TargetBits, "targetBits", _TargetBits, "target bits")
	flag.Parse()
	log.Println("Target bits:", _TargetBits)

	if err = os.MkdirAll("data", 0755); err != nil {
		log.Fatalln(err)
	}

	defer func() {
		if err != nil {
			log.Fatalln(err)
		}
	}()

	if bc, err = NewBlockchain("data/blockchain.db", "theBlockchain"); err != nil {
		log.Fatalf("NewBlockchain: %v\n", err)
	}
	defer bc.Close()

	if err = bc.AddBlock("Send 1 BTC to Ivan"); err != nil {
		return
	}
	if err = bc.AddBlock("Send 2 more BTC to Ivan"); err != nil {
		return
	}

	iter := bc.Iterator()
	for {
		if block := iter.Next(); block == nil {
			break
		} else {
			count++
			fmt.Println(">>>", strings.Replace(block.String(), ", ", "\n    ", -1))
		}
	}

	fmt.Printf("~~~ %d blocks in total\n", count)
}
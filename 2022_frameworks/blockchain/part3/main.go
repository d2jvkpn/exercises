// https://jeiwan.net/posts/building-blockchain-in-go-part-2/
package main

import (
	"flag"
	"fmt"
	"log"
	"os"
)

var (
	_TargetBits int = 20
)

func main() {
	var (
		err error
		bc  *Blockchain
	)

	flag.IntVar(&_TargetBits, "targetBits", _TargetBits, "target bits")
	flag.Parse()
	fmt.Println("~~~ target bits:", _TargetBits)

	if err = os.MkdirAll("data", 0755); err != nil {
		log.Fatalln(err)
	}

	if bc, err = NewBlockchain("data/blockchain.db", "theBlockchain"); err != nil {
		log.Fatalf("NewBlockchain: %v\n", err)
	}

	_ = bc.AddBlock("Send 1 BTC to Ivan")
	_ = bc.AddBlock("Send 2 more BTC to Ivan")

	iter := bc.Iterator()
	for {
		if block := iter.Next(); block == nil {
			break
		} else {
			fmt.Println(block)
		}
	}
}

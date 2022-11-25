// https://jeiwan.net/posts/building-blockchain-in-go-part-2/
package main

import (
	"flag"
	"fmt"
)

var (
	_TargetBits = 20 // 24
)

func main() {
	flag.IntVar(&_TargetBits, "targetBits", 20, "target bits")
	flag.Parse()

	fmt.Println("~~~ target bits:", _TargetBits)
	bc := NewBlockchain().AddBlock("Send 1 BTC to Ivan").AddBlock("Send 2 more BTC to Ivan")

	for i := range bc.blocks {
		fmt.Println(bc.blocks[i])
	}

	for idx, block := range bc.blocks {
		pow := NewProofOfWork(block)
		fmt.Printf("Block: %d, PoW: %t\n", idx, pow.Validate())
	}
}

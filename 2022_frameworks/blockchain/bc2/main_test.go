package main

import (
	"fmt"
	"math/big"
	"testing"
)

func TestBigint(t *testing.T) {
	var val big.Int

	fmt.Printf("%v, 'A'=%d, 'B'=%d\n", val, 'A', 'B')

	val.SetBytes([]byte{1, 1, 1})
	fmt.Printf("%v\n", val)

	// val.SetBytes([]byte{'A', 'B'})
	val.SetBytes([]byte{1})
	fmt.Printf("%v\n", val)

	val.SetBytes([]byte{255, 255})
	fmt.Printf("%v\n", val)
}

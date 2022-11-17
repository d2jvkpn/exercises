package main

import (
	"fmt"
	"math/big"
	"testing"
)

func TestBigint(t *testing.T) {
	var val big.Int

	fmt.Printf("~~~ %d, %d, %d, %d\n", 0, '0', 'A', 'a')

	fmt.Printf("%v, 'A'=%[2]d, %[2]b, 'B'=%[3]d, %[3]b\n", val, 'A', 'B')
	val.SetBytes([]byte{'A', 'B'})
	fmt.Printf("%v, %d, %d, %v\n", val, val.Uint64(), 'A'<<8+'B', IntToHex(16706))

	val.SetBytes([]byte{1, 1, 1})
	fmt.Printf("%v\n", val)

	val.SetBytes([]byte{1})
	fmt.Printf("%v\n", val)

	val.SetBytes([]byte{255, 255})
	fmt.Printf("%v\n", val)
}

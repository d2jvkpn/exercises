// nil.go
package main

import (
	"fmt"
)

func main() {
	// n := nil // compile failed
	var n map[int]int
	fmt.Println(n, n == nil)
}

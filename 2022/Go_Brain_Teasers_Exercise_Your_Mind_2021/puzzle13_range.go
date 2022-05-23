// range.go
package main

import (
	"fmt"
)

func fibs(n int) chan int {
	ch := make(chan int)
	go func() {
		a, b := 1, 1
		for i := 0; i < n; i++ {
			ch <- a
			a, b = b, a+b
		}
		close(ch)
	}()

	return ch
}

func main() {
	for i := range fibs(5) {
		fmt.Printf("%d ", i)
	}
	// for { i, ok := <-ch; ... }

	fmt.Println()
}

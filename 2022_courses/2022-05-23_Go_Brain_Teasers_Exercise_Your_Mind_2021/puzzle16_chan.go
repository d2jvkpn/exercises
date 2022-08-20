// chan.go
package main

import (
	"fmt"
)

func main() {
	ch := make(chan int, 2)
	ch <- 1
	ch <- 2
	<-ch
	close(ch)
	a := <-ch
	// a, ok := <-ch // 2, true
	b := <-ch
	// b, ok := <-ch // 0, false
	fmt.Println(a, b)
}

/*
output: 2 0
*/

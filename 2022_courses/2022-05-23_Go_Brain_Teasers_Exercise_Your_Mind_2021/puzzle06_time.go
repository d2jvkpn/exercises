// time.go
package main

import (
	"fmt"
	"time"
)

func main() {
	timeout := 3
	fmt.Printf("before ")
	// time.Sleep(timeout * time.Millisecond)
	time.Sleep(time.Duration(timeout) * time.Millisecond)
	fmt.Println("after")
}

/*
a type conversion to convert const int64 to time.Duration, but won't compile a variable int64 to
time.Duration
*/

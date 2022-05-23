// raw.go
package main

import (
	"fmt"
)

func main() {
	s := `a\tb`
	fmt.Println(s)

	fmt.Println("\u2122")
}

/*
output: a\tb

In raw strings, \ has no special meaning, so the '\t' is two characters
*/

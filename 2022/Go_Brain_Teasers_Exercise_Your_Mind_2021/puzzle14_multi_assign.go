// multi_assign.go
package main

import (
	"fmt"
)

func main() {
	a, b := 1, 2
	b, c := 3, 4
	fmt.Println(a, b, c)
}

/*
output: 1 3 4

b is an existing variable, and the type of 3 that is int matches the current type
of b . OK here. c is a new variable, making this statement OK.
*/

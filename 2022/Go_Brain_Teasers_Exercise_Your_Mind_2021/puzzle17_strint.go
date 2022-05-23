// strint.go
package main

import (
	"fmt"
	"strconv"
)

func main() {
	i := 169
	s := string(i)
	fmt.Println(s)

	s = strconv.Itoa(i)
	fmt.Println(s)

	data := []byte{'1', '2', '9'}
	s = string(data)
	fmt.Println(s) // 129
}

/*
output:
©
169
129
*/

// runes.go
package main

import (
	"fmt"
)

func main() {
	msg := " π = 3.14159265358..."
	fmt.Printf("%T ", msg[0]) // byte or uint8

	for _, c := range msg { // unint32
		fmt.Printf("%T\n", c)
		break
	}

	fmt.Println(len(msg), len([]rune(msg)))
}

/*
output:
uint8 int32
22 21
*/

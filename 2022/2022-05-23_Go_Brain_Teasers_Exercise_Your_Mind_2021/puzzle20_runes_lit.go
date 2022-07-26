// runes_lit.go
package main

import (
	"fmt"
)

func main() {
	r1 := '™'
	fmt.Printf("r1 = %[1]v, %[1]c\n", r1) // 8482, ™

	r2 := '\x61'                          // \x - 2 digits
	fmt.Printf("r2 = %[1]v, %[1]c\n", r2) // a

	r3 := '\u2122'                        // \u - 4 digits (8482 in hex)
	fmt.Printf("r3 = %[1]v, %[1]c\n", r3) // ™

	r4 := '\U00002122'                    // \U - 8 digits
	fmt.Printf("r4 = %[1]v, %[1]c\n", r4) // ™
}

// flag_ok.go
package main

import (
	"fmt"
)

func main() {
	type Flag int
	type Flag2 = int

	var i interface{} = 3
	f, ok := i.(Flag) // f := i.(Flag) will cause panic
	if !ok {
		fmt.Println(">>> 1 not a Flag")
	} else {
		fmt.Println(">>> 1", f)
	}

	// v := Flag(i) // can't compile: cannot convert i (variable of type interface{}) to type Flag:

	fmt.Println(i.(Flag2)) // 3
}

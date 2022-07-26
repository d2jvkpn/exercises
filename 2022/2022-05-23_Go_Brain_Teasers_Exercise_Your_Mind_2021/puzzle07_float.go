// float.go
package main

import (
	"fmt"
	"math"
)

func main() {
	n := 1.1
	fmt.Println(n * n)

	fmt.Println(math.NaN() == math.NaN())
}

/*
1.2100000000000002

Floating point is sort of like quantum physics: the closer you look, the messier it gets.
— Grant Edwards
*/

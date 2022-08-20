// two_cities_nfc.go
package main

import (
	"fmt"

	"golang.org/x/text/unicode/norm"
)

func main() {
	city1, city2 := "Kraków", "Kraków"
	fmt.Println(city1 == city2)
	city1, city2 = norm.NFC.String(city1), norm.NFC.String(city2)
	fmt.Println(city1 == city2)
}

/*
output:
false
true
*/

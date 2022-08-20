// city.go
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	abc := "abc"
	city := "Kraków"
	fmt.Println(len(abc), len(city), len([]byte(city)), len([]rune(city)))
	fmt.Println(utf8.RuneCountInString(city))
}

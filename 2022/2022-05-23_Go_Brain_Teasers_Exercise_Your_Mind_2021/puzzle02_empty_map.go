// empty_map.go
package main

import (
	"fmt"
)

func main() {
	var m map[string]int
	fmt.Println(m["errors"], len(m)) //
}

/*
- Some operations on Go’s map type are nil safe, meaning they will work with a nil map without panicking.
- assign value on nil map will cause panic: m["ok"] = 1
- make(map[?]?, 100) creates a map with capacity, but you can't cap(m)
*/

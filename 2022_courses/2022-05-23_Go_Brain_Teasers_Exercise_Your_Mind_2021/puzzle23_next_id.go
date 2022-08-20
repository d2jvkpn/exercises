// next_id.go
package main

import (
	"fmt"
)

var (
	id = nextID()
)

func nextID() int {
	id++
	return id
}

// $ go build puzzle23_next_id.go
func main() {
	fmt.Println(id)
}

/*
# command-line-arguments
./puzzle23_next_id.go:9:2: initialization loop for id
	./puzzle23_next_id.go:9:2: id refers to
	./puzzle23_next_id.go:12:6: nextID refers to
	./puzzle23_next_id.go:9:2: id
*/

package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello, world!")
}

/*
v1: queue: []string
value := ["a", "b", "c", None]

push "d": ["a", "b", "c", "d"]
pop: ["a", "b", "c", "d"][1:], got "a"
push "e": ["e", "b", "c", "d"][0:]
*/

/*
v1: queue: [][]string
value := [[None, None, None]]

push "a", "b", "c": [["a", "b", "c"]]
push "d": [["a", "b", "c"], ["d", None, None]]
pop 1: [["a", "b", "c"][1:], ["d", None, None]], got "a"
pop 2: [["d", None, None]], got "b", "c"
*/

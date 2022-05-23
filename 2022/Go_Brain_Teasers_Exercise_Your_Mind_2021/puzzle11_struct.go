// struct.go
package main

import (
	"fmt"
	"io"
	"time"
)

// Log is a log message
type Log struct {
	Message string
	time.Time
}

// Point is a 2D point
type Point struct {
	X int
	Y int
}

func main() {
	p := Point{1, 2}
	fmt.Printf("%+v\n", p)

	ts := time.Date(2009, 11, 10, 0, 0, 0, 0, time.UTC)
	log := Log{"Hello", ts}
	fmt.Printf("%+v\n", log)
}

/*
time.Time defines a String() string method, which means it implements the [ fmt.Stringer
interface. And since Log embeds time.Time it also has a String() string method. If a
parameter passed to fmt fmt.Printf implements fmt.Stringer , fmt.Printf will use it instead of
the default output.
*/

type ReadWriter interface {
	io.Reader
	io.Writer
}

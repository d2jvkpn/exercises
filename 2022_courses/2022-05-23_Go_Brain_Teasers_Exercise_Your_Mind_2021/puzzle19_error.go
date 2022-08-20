// error.go
package main

import (
	"fmt"
)

type OSError int

func (e *OSError) Error() string {
	return fmt.Sprintf("error #%d", *e)
}

func FileExists(path string) (bool, error) {
	var err *OSError
	return false, err // TODO
}

func main() {
	if _, err := FileExists("/no/such/file"); err != nil {
		fmt.Printf("error: %[1]s, %[1]T, %[1]v\n", err)
	} else {
		fmt.Println("OK")
	}
}

/*
output: error: <nil>, *main.OSError, <nil>

If you look at src/runtime/runtime2.go in the Go source repository, you’ll see the
following definition:

type iface struct {
	tab *itab
	data unsafe.Pointer
}

• itab describes the interface.
• data is a pointer to the value that implements the interface.
*/

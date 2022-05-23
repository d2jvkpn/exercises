// error.go
package main

import (
	"fmt"
)

type OSError int

func (e OSError) Error() string {
	return fmt.Sprintf("error #%d", e)
}

func FileExists(path string) (bool, error) {
	var err OSError
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
output: error: error #0, main.OSError, error #0
*/

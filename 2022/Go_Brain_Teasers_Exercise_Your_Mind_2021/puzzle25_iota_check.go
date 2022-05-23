// iota_check.go
package main

import (
	"fmt"
)

const (
	Read = 1 << iota
	Write
	Execute
)

func main() {
	mask := Read | Execute

	fmt.Printf(">>> Read=%b, Write=%b, Execute=%b, mask=%b\n", Read, Write, Execute, mask)

	if mask&Execute == 0 {
		fmt.Println("can't execute")
	} else { // 100
		fmt.Println("can execute") // will be printed
	}

	if mask&Write == 0 { // 0
		fmt.Println("can't write") // will be printed
	} else {
		fmt.Println("can write")
	}
}

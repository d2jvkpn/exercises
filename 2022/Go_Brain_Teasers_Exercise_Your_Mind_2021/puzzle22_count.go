// count.go
package main

import (
	"fmt"
	"sync"
)

// $ go run -race puzzle22_count.go
func main() {
	var count int
	var wg sync.WaitGroup
	for i := 0; i < 1_000_000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			count++
		}()
	}
	wg.Wait()
	fmt.Println(count)
}

/*
output:
!!! not 1000000
*/

// sleep_sort_param.go
package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	for _, n := range []int{3, 1, 2} {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			time.Sleep(time.Duration(n) * time.Millisecond)
			fmt.Printf("%d ", i)
		}(n)
	}
	wg.Wait()
	fmt.Println()
}

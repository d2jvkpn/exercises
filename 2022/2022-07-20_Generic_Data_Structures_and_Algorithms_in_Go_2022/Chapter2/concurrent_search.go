package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"time"
)

type Ordered interface {
	~float64 | ~int64 | ~string
}

const size = 100_000_000

var (
	ch     chan bool
	cancel chan struct{}
)

func init() {
	ch = make(chan bool)
	cancel = make(chan struct{})
}

func searchSegment[T Ordered](slice []T, target T, a, b int) {
	var found bool
	// Generates boolean value put into ch
LOOP:
	for i := a; i < b; i++ {
		select {
		case <-cancel:
			return
		default:
			if slice[i] == target {
				found = true
				break LOOP
			}
		}

	}

	select {
	case <-cancel:
	case ch <- found:
	}
}

func concurrentSearch[T Ordered](data []T, target T) bool {
	numSegments := runtime.NumCPU()
	numSegments = 15
	segmentSize := len(data) / numSegments

	fmt.Printf(
		"~~~ len(data)=%d, numSegments=%d, segmentSize=%d\n",
		len(data), numSegments, segmentSize,
	)
	// Launch numSegments goroutines
	segs := make([][2]int, numSegments)
	for i := 0; i < numSegments; i++ {
		a, b := i*segmentSize, i*segmentSize+segmentSize
		segs[i] = [2]int{a, b}
	}

	if segs[len(segs)-1][1] < len(data) {
		segs[len(segs)-1][1] = len(data)
	}

	for _, v := range segs {
		fmt.Printf("    segment=%v, size=%d\n", v, v[1]-v[0])
		go searchSegment(data, target, v[0], v[1])
	}

	num := 0 // Completed goroutines

	for {
		select {
		case value := <-ch: // Blocks until a goroutine puts a bool into the
			//channel
			if value {
				close(cancel)
				return true
			}
			num += 1

			if num == numSegments { // All goroutiines have completed
				return false
			}
		}
	}
}

func main() {
	data := make([]float64, size)
	for i := 0; i < size; i++ {
		data[i] = 100.0 * rand.Float64()
	}

	start := time.Now()
	result := concurrentSearch[float64](data, 54.0) // Should return false
	elapsed := time.Since(start)
	fmt.Println("Time to search slice of 100_000_000 floats using concurrentSearch = ", elapsed)
	fmt.Println("Result of search is ", result)

	start = time.Now()
	result = concurrentSearch[float64](data, data[size/2]) // true
	elapsed = time.Since(start)
	fmt.Println("Time to search slice of 100_000_000 floats using concurrentSearch = ", elapsed)
	fmt.Println("Result of search is ", result)
}

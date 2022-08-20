package main

import (
	"fmt"
	"time"
)

const size = 100_000_000

type Ordered interface {
	~float64 | ~int64 | ~string
}

func binarySearch[T Ordered](slice []T, target T) int {
	if len(slice) == 0 {
		return -1
	}

	median, low, high := 0, 0, len(slice)-1
	for low < high {
		median := (low + high) / 2
		if slice[median] < target {
			low = median + 1
		} else {
			high = median - 1
		}
	}

	switch target {
	case slice[low]:
		return low
	case slice[median]:
		return median
	case slice[high]:
		return high
	default:
		return -1
	}
}

func main() {
	data := make([]float64, size)
	for i := 0; i < size; i++ {
		data[i] = float64(i) // is sorted
	}
	start := time.Now()
	result := binarySearch[float64](data, -10.0)
	elapsed := time.Since(start)
	fmt.Println("Time to search slice of 100_000_000 floats using binarySearch = ", elapsed)

	fmt.Println("Result of search is ", result)
	start = time.Now()
	result = binarySearch[float64](data, float64(size/2))
	elapsed = time.Since(start)
	fmt.Println("Time to search slice of 100_000_000 floats using binarySearch = ", elapsed)
	fmt.Println("Result of search is ", result)
}

package main

import (
	"fmt"
	"math/rand"
	"runtime"
	// "sort"
	"time"
)

var data []float64

func isSegmentSorted(data []float64, a, b int, ch chan<- bool) {
	// Generates boolean value put into ch
	for i := a + 1; i < b; i++ {
		if data[i] < data[i-1] {
			ch <- false
		}
	}
	ch <- true
}

func isSorted2(data []float64) bool {
	for i := 0; i < len(data)-1; i++ {
		if data[i] > data[i+1] {
			return false
		}
	}
	return true
}

func isSorted3(data []float64) bool {
	ch := make(chan bool)
	numSegments := runtime.NumCPU()
	segmentSize := int(float64(len(data)) / float64(numSegments))
	fmt.Printf("~~~ numSegments=%v, segmentSize=%v\n", numSegments, segmentSize)
	// Launch numSegments goroutines
	for index := 0; index < numSegments; index++ {
		go isSegmentSorted(data, index*segmentSize,
			index*segmentSize+segmentSize, ch)
	}
	num := 0 // Completed goroutines
	for {
		select {
		case value := <-ch:
			if value == false {
				return false
			}
			num += 1
			if num == numSegments { // All goroutiines have completed
				return true
			}
		}
	}
}

func main() {
	const size = 1_000
	data = make([]float64, size)
	for i := 0; i < size; i++ {
		data[i] = 100.0 * rand.Float64()
	}
	data2 := make([]float64, size)
	// Create a sorted sequence of numbers
	for i := 0; i < size; i++ {
		data2[i] = float64(2 * i)
	}

	start := time.Now()
	result := isSorted2(data)
	elapsed := time.Since(start)
	fmt.Printf("Sorted2: %t, %v\n", result, elapsed)

	start = time.Now()
	result = isSorted3(data2)
	elapsed = time.Since(start)
	fmt.Printf("Sorted3: %t, %v\n", result, elapsed)
}

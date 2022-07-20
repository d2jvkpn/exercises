package main

import (
	"fmt"
	"math/rand"
	"time"
)

// const size = 50_000_000
const size = 500

type Ordered interface {
	~float64 | ~int64 | ~string
}

func IsSorted[T Ordered](data []T) bool {
	for i := 1; i < len(data); i++ {
		if data[i] < data[i-1] {
			return false
		}
	}
	return true
}

func InsertSort[T Ordered](data []T) {
	// fmt.Println("====")
	i := 1
	for i < len(data) {
		h := data[i]
		j := i - 1
		for j >= 0 && h < data[j] {
			data[j+1] = data[j]
			j -= 1
		}
		data[j+1] = h
		i += 1
	}
}

func Merge[T Ordered](left, right []T) []T {
	// fmt.Println("~~~~")
	result := make([]T, len(left)+len(right))
	i, j, k := 0, 0, 0

	for ;i < len(left) && j < len(right); k++ {
		if left[i] < right[j] {
			result[k] = left[i]
			i++
		} else {
			result[k] = right[j]
			j++
		}
	}

	for ;i < len(left); k++ {
		result[k] = left[i]
		i++
	}

	for ;j < len(right); k++ {
		result[k] = right[j]
		j++
	}

	return result
}

func MergeSort[T Ordered](data []T) []T {
	if len(data) > 100 {
		middle := len(data) / 2
		left, right := data[:middle], data[middle:]
		data = Merge(MergeSort(right), MergeSort(left))
	} else {
		InsertSort(data)
	}

	return data
}

func main() {
	data := make([]float64, size)
	for i := 0; i < size; i++ {
		data[i] = 100.0 * rand.Float64()
	}
	/*
		data2 := make([]float64, size)
		copy(data2, data)
	*/

	start := time.Now()
	result := MergeSort[float64](data)
	elapsed := time.Since(start)
	fmt.Printf("Elapsed time for MergeSort = %v, Is sorted: %t\n", elapsed, IsSorted(result))
}

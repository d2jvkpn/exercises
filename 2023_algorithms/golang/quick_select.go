package goimpls

import (
// "fmt"
)

func quickSelectPartition(arr []int, low, high int) int {
	if low >= high {
		return low
	}

	index := low
	pivot := arr[high]

	for i := low; i < high; i++ {
		if arr[i] <= pivot {
			// fmt.Printf("~~~ i=%d, index=%d, arr[i]=%d\n", i, index, arr[i])
			if index != i {
				arr[index], arr[i] = arr[i], arr[index]
			}
			index += 1
		}
	}
	arr[index], arr[high] = arr[high], arr[index]

	return index
}

func quickSelectRange(arr []int, k, low, high int) int {
	if len(arr) == 0 {
		panic("empty array")
	} else if len(arr) == 1 {
		return 0
	}

	if low >= high {
		return high
	}

	pivot := quickSelectPartition(arr, low, high)
	switch {
	case k < pivot:
		return quickSelectRange(arr, k, low, pivot-1)
	case k > pivot:
		return quickSelectRange(arr, k, pivot+1, high)
	default:
		return pivot
	}
}

func QuickSelect(arr []int, k int) int {
	return quickSelectRange(arr, k, 0, len(arr)-1)
}

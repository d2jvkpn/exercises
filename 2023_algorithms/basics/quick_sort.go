package main

import (
	"fmt"
)

func partition(arr []int, low, high int) int {
	index := low - 1
	pivotElement := arr[high]

	for i := low; i < high; i++ {
		if arr[i] <= pivotElement {
			index += 1
			if index != i {
				arr[index], arr[i] = arr[i], arr[index]
			}
		}
	}

	arr[index+1], arr[high] = arr[high], arr[index+1]
	return index + 1
}

func quickSortRange(arr []int, low, high int) {
	if len(arr) <= 1 {
		return
	}

	if low < high {
		pivot := partition(arr, low, high)
		// fmt.Println("==>", pivot, arr)
		quickSortRange(arr, low, pivot-1)
		quickSortRange(arr, pivot+1, high)
	}
}

func QuckSort(arr []int) {
	quickSortRange(arr, 0, len(arr)-1)
}

func main() {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	QuckSort(arr)

	fmt.Println(arr)
}

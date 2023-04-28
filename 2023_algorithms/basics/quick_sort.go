package main

import (
	"fmt"
)

func partition(arr []int, low, high int) int {
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

func quickSortRange(arr []int, low, high int) {
	if len(arr) <= 1 {
		return
	}

	if low < high {
		pivotIndex := partition(arr, low, high)
		fmt.Println("~~>", arr, low, high, pivotIndex)
		quickSortRange(arr, low, pivotIndex-1)
		quickSortRange(arr, pivotIndex+1, high)
	}
}

func QuckSort(arr []int) {
	quickSortRange(arr, 0, len(arr)-1)
}

func main() {
	arr1 := []int{3, 1, 4, 7, 5, 9, 2, 6, 8, 5}	
	fmt.Println("==>", arr1)
	QuckSort(arr1)
	fmt.Println(arr1)

	arr2 := []int{7, 2, 1, 9, 3, 6, 8}
	fmt.Println("==>", arr2)
	QuckSort(arr2)
	fmt.Println(arr2)
}
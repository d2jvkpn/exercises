package go_impls

import (
	"fmt"
	"testing"
)

func TestHeapSort(t *testing.T) {
	data := []int{12, 11, 13, 5, 6, 7, 20, 6, 7, 3}
	fmt.Println("Unsorted array:", data)

	heapSort(data)
	fmt.Println("Sorted array:", data)
}

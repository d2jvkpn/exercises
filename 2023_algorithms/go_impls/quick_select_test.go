package go_impls

import (
	"fmt"
	"testing"
)

func TestQuickSort(t *testing.T) {
	arr1 := []int{3, 1, 4, 7, 5, 9, 2, 6, 8, 5}
	fmt.Println("==>", arr1)
	QuckSort(arr1)
	fmt.Println(arr1)

	arr2 := []int{7, 2, 1, 9, 3, 6, 8}
	fmt.Println("==>", arr2)
	QuckSort(arr2)
	fmt.Println(arr2)
}

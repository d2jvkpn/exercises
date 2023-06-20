package go_impls

import (
	"fmt"
	"testing"
)

func TestQuickSelect(t *testing.T) {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
	k := QuickSelect(arr, 5)

	fmt.Println(arr[:k])
}

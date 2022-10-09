package main

import (
	"fmt"
	"testing"
)

func TestMaxSubseq(t *testing.T) {
	nums := []int{10, 5, -2, -2, 20}
	fmt.Println(MaxSubseq(nums, 2))
}

func TestSortSeq(t *testing.T) {
	n := 1000
	out := SortSeq(n)
	fmt.Printf(">>> n=%d, len=%d:\n%v\n", n, len(out), out)
}

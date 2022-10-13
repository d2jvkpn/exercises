package main

import (
	"fmt"
	"testing"
)

func TestMerge(t *testing.T) {
	nums1 := []int{1, 2, 3, 0, 0, 0}
	nums2 := []int{2, 5, 6}

	Merge(nums1, 3, nums2, 3)
	fmt.Println(nums1)

	nums1 = []int{-1, 0, 0, 3, 3, 3, 0, 0, 0}
	nums2 = []int{1, 2, 2}

	Merge(nums1, 6, nums2, 3)
	fmt.Println(nums1)

	nums1 = []int{0}
	nums2 = []int{1}
	Merge(nums1, 0, nums2, 1)
	fmt.Println(nums1)

	nums1 = []int{4, 0, 0, 0, 0, 0}
	nums2 = []int{1, 2, 3, 5, 6}
	Merge(nums1, 1, nums2, 5)
	fmt.Println(nums1)
}

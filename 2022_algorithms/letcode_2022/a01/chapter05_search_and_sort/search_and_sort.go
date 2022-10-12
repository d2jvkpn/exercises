package main

import (
	"fmt"
)

// nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// m = len(nums1) - len(nums2); n = len(nums2)
func Merge(nums1 []int, m int, nums2 []int, n int) {
	fmt.Println(">>>", nums1, nums2)

	i, j := 0, 0
	for i < m+n && j < n {
		if nums1[i] > nums2[j] {
			nums1 = append(nums1[:i+1], nums1[i:m+n-1]...)
			nums1[i] = nums2[j]
			j++
		}
		i++
	}

	if j < n {
		fmt.Println("   ", j, nums1[:m+j], nums2[j:])
		nums1 = append(nums1[:m+j], nums2[j:]...)
	}
}

func isBadVersion(version int) bool {
	return version >= 4
}

func firstBadVersion(n int) int {
	lower, upper := 1, n

	for {
		mid := (lower + upper) / 2
		isBad := isBadVersion(mid)

		if isBad && mid == 1 {
			return mid
		}
		if !isBad && isBadVersion(mid+1) {
			return mid + 1
		}

		if isBad {
			upper = mid
		} else {
			lower = mid
		}
	}

	return -1
}

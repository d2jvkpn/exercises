package main

import (
// "fmt"
)

// len nums > 2; k > 0 && k < len(nums)
func MaxSubseq(nums []int, k int) int {
	min := nums[0]

	sum := func(arr []int) int {
		out := 0
		for i := range arr {
			out += arr[i]
		}
		return out
	}

	for i := 0; i < len(nums)-k; i++ {
		for j := 0; j < k+1; j++ {
			s := sum(nums[i : i+j])
			if s < min {
				min = s
			}
		}
	}

	return sum(nums) - min
}

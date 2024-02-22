package goimpls

import (
// "fmt"
)

func partition(nums []int, low, high int) int {
	if low >= high {
		return low
	}

	pivot, value := low, nums[high]

	for i := low; i < high; i++ {
		if nums[i] <= value {
			// fmt.Printf("~~~ i=%d, index=%d, nums[i]=%d\n", i, index, nums[i])
			if pivot != i {
				nums[pivot], nums[i] = nums[i], nums[pivot]
			}
			pivot += 1
		}
	}

	nums[pivot], nums[high] = nums[high], nums[pivot]

	return pivot
}

func qs(nums []int, low, high int) {
	if low >= high {
		return
	}

	pivot := partition(nums, low, high)
	// fmt.Println("~~>", nums, low, high, pivot)

	if low+1 < pivot {
		qs(nums, low, pivot-1)
	}

	if pivot+1 < high {
		qs(nums, pivot+1, high)
	}
}

func QuckSort(nums []int) {
	if len(nums) < 2 {
		return
	}

	qs(nums, 0, len(nums)-1)
}

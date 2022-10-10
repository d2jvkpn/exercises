package main

import (
	"fmt"
)

func ContainsDuplicate(nums []int) bool {
	if len(nums) < 2 {
		return false
	}

	for i := 0; i < len(nums)-1; i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i] == nums[j] {
				return true
			}
		}
	}
	return false
}

// nums is in sorted in ascending order
func RemoveDuplicates(nums []int) int {
	if len(nums) < 2 {
		return len(nums)
	}

	//	for i := 0; i < len(nums)-1; i++ {
	//		for j := i+1; j < len(nums); j++ {
	//			if nums[i] == nums[j] {
	//				nums = append(nums[:j], nums[j+1:]...)
	//				j--
	//			}
	//		}
	//	}

	for i := 0; i < len(nums)-1; i++ {
		if nums[i] == nums[i+1] {
			nums = append(nums[:i], nums[i+1:]...)
			i--
		}
	}

	return len(nums)
}

func RotateX1(nums []int, k int) []int {
	k = k % len(nums)
	if k == 0 {
		return nums
	}

	z := len(nums) - k
	fmt.Printf("~~~ %d, %v\n", z, nums)
	nums = append(nums[z:], nums[:z]...)
	fmt.Printf("    %v\n", nums)

	return nums
}

func Rotate(nums []int, k int) {
	k = k % len(nums)
	if k == 0 {
		return
	}

	z := len(nums) - k
	tmp := make([]int, 0, k)
	for i := z; i < len(nums); i++ {
		tmp = append(tmp, nums[i])
	}

	for i := z - 1; i > -1; i-- {
		nums[i+k] = nums[i]
	}

	for i := range tmp {
		nums[i] = tmp[i]
	}
}

func SingleNumber(nums []int) int {
	for i := 0; i < len(nums); i++ {
		isDup := false
		for j := 0; j < len(nums); j++ {
			if i == j {
				continue
			}
			if nums[i] == nums[j] {
				isDup = true
				break
			}
		}
		if !isDup {
			return nums[i]
		}
	}

	return nums[len(nums)-1]
}

func Intersect(nums1 []int, nums2 []int) []int {
	m1 := make(map[int]int, len(nums1))
	m2 := make(map[int]int, len(nums2))

	for i := range nums1 {
		m1[nums1[i]]++
	}
	for i := range nums2 {
		m2[nums2[i]]++
	}

	out := make([]int, 0)
	for k := range m1 {
		for m1[k] > 0 {
			if m2[k] > 0 {
				out = append(out, k)
				m2[k]--
				m1[k]--
			} else {
				break
			}
		}
	}

	return out
}

// all elements in digits are non-zero
func PlusOne(digits []int) []int {
	v := 0

	for i := len(digits) - 1; i > -1; i-- {
		v = digits[i] + 1
		if v < 10 {
			digits[i] = v
			break
		}

		digits[i] = v % 10
		if i == 0 {
			break
		}
	}

	if v >= 10 {
		digits = append([]int{1}, digits...)
	}

	return digits
}

func MoveZeroes(nums []int) {
	for i := 0; i < len(nums); i++ {
		if nums[i] != 0 {
			continue
		}

		found := false
		for j := i + 1; j < len(nums); j++ {
			if nums[j] != 0 {
				nums[i], nums[j] = nums[j], nums[i]
				found = true
				break
			}
		}
		if !found {
			break
		}
	}
}

func TwoSum(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}

	return []int{-1, -1}
}

func RotateMatrix(matrix [][]int) {
	n := len(matrix)
	newM := make([][]int, 0, n)

	for i := 0; i < n; i++ {
		vec := make([]int, n)
		for j := 1; j <= n; j++ {
			vec[j-1] = matrix[n-j][i]
		}
		newM = append(newM, vec)
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			matrix[i][j] = newM[i][j]
		}
	}
}

func IsValidSudoku(board [][]byte) bool {
	validArr := func(bts []byte) bool {
		for i := 0; i < len(bts); i++ {
			for j := i + 1; j < len(bts); j++ {
				if bts[i] == '.' || bts[j] == '.' {
					continue
				}
				if bts[i] == bts[j] {
					return false
				}
			}
		}
		return true
	}

	validXY := func(mat [][]byte) bool {
		n := len(mat)

		for i := 0; i < n; i++ {
			if !validArr(mat[i]) {
				return false
			}
			vec := make([]byte, n)
			for j := 0; j < n; j++ {
				vec[j] = mat[j][i]
			}
			if !validArr(vec) {
				return false
			}
		}
		return true
	}

	valid33 := func(mat [][]byte) bool {
		d := make(map[byte]bool, 9)
		for i := range mat {
			for _, v := range mat[i] {
				if v == '.' {
					continue
				}
				if d[v] {
					return false
				}
				d[v] = true
			}
		}
		return true
	}

	if !validXY(board) {
		return false
	}

	for i := 0; i < len(board)/3; i++ {
		for j := 0; j < len(board)/3; j++ {
			n, m := i*3, j*3
			// fmt.Println("~~~", n, m)
			newM := [][]byte{
				board[n][m : m+3],
				board[n+1][m : m+3],
				board[n+2][m : m+3],
			}
			if !valid33(newM) {
				return false
			}
		}
	}

	return true
}

func MaxProfit(prices []int) int {
	max := 0

	for i := 0; i < len(prices)-1; i++ {
		if prices[i] < prices[i+1] {
			max += prices[i+1] - prices[i]
		}
	}

	return max
}

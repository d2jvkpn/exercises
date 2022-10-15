package main

import (
// "fmt"
)

/*
https://www.tutorialcup.com/leetcode-solutions/01-matrix-leetcode-solution.htm

0 0 0    0 0 0
0 1 0 => 0 1 0
0 0 0    0 0 0

0 0 0    0 0 0
0 1 0 => 0 1 0
1 1 1    1 2 1
*/

func NearestZeroDist(mat [][]int) (out [][]int) {
	m, n := len(mat), len(mat[0])
	out = make([][]int, 0, m)

	get := func(i, j int) int {
		if i < 0 || i >= m {
			return -1
		}
		if j < 0 || j >= n {
			return -1
		}
		return out[i][j]
	}

	fill := func(i, j int) bool {
		if out[i][j] >= 0 {
			return true
		}

		arr := make([]int, 0, 4)
		for _, v := range []int{get(i-1, j), get(i+1, j), get(i, j-1), get(i, j+1)} {
			if v > -1 {
				arr = append(arr, v)
			}
		}
		if len(arr) == 0 {
			return false
		}

		for i := 1; i < len(arr)-1; i++ {
			if arr[i] < arr[0] {
				arr[0], arr[i] = arr[i], arr[0]
			}
		}

		out[i][j] = arr[0] + 1
		return true
	}

	for i := 0; i < m; i++ {
		vec := make([]int, n)
		for j := 0; j < n; j++ {
			if mat[i][j] == 0 {
				vec[j] = 0
			} else {
				vec[j] = -1
			}
		}
		out = append(out, vec)
	}

	for {
		ok := true
		for i := 0; i < m; i++ {
			for j := 0; j < n; j++ {
				if z := fill(i, j); !z {
					ok = false
				}
			}
		}
		if ok {
			break
		}
	}

	return out
}

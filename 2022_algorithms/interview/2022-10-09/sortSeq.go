package main

import (
	"fmt"
)

// n > 0
func SortSeq(n int) (out []int) {
	out = make([]int, 0, n)

	firstNum := func(v int) int {
		for v > 9 {
			v /= 10
		}
		return v
	}

	for i := 1; i < 10 && i < n; i++ {
		out = append(out, i)

	LOOP:
		for v := i * 10; v <= n; v += 10 {
			//  if fmt.Sprintf("%d", i)[0] != fmt.Sprintf("%d", v)[0] {
			//  	continue
			//  }
			if firstNum(v) != firstNum(i) {
				continue
			}
			fmt.Println("~~~ v =", v)
			for j := 0; j < 10; j++ {
				z := v + j
				// fmt.Println("~~~", z)
				if z > n {
					break LOOP
				}
				out = append(out, z)
			}
		}
	}

	return out
}

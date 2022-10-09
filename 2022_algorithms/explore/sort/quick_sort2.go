package explore

import (
	"fmt"
)

func QuickSort2(slice []int) (out []int) {
	if len(slice) < 2 {
		return slice
	}

	s1, s2 := make([]int, 0, len(slice)/2), make([]int, 0, len(slice)/2)

	for i := 1; i < len(slice); i++ {
		if slice[i] < slice[0] {
			s1 = append(s1, slice[i])
		} else {
			s2 = append(s2, slice[i])
		}
	}
	fmt.Printf("    s1 = %v, s2 = %v\n", s1, s2)

	out = make([]int, 0, len(slice))
	out = append(QuickSort2(s1), slice[0]) // recursion
	out = append(out, QuickSort2(s2)...)   // recursion

	return
}

func InstQuickSort2() {
	fmt.Println(">>> InstQuickSort:")
	slice := []int{14, 33, 10, 27, 19, 35, 42, 44, 18}
	fmt.Printf("    slice = %v\n", slice)

	out := QuickSort2(slice)
	fmt.Printf("    out = %v\n", out)
}

/*
  14, 33, 10, 27, 19, 35, 42, 44, 18

  [10], 14, [33, 27, 19, 35, 44, 18]
  [10], 14, [[27, 19, 18], 33, [35, 42, 44]]
  [10], 14, [[19, 18], 27], 33, [35, [42, 44]]
  [10], 14, [[18, [19]], 27], 33, [35, [42, 44]]
  [10], 14, [[18, 19], 27], 33, [35, 42, 44]
  [10], 14, [[18, 19, 27], 33, 35, 42, 44]
  [10], 14, [[18, 19, 27], 33, 35, 42, 44]
  [10], 14, [18, 19, 27, 33, 35, 42, 44]
  [10, 14, 18, 19, 27, 33, 35, 42, 44]
*/

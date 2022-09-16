package explore

import (
	"fmt"
)

func SelectSort(slice []int) (n int) {
	for i := 0; i < len(slice)-1; i++ {
		z := i
		for j := i + 1; j < len(slice); j++ {
			if slice[j] < slice[i] {
				z = j
			}
		}

		if z != i {
			n++
			slice[i], slice[z] = slice[z], slice[i]
		}
	}

	return
}

func InstSelectSort() {
	fmt.Println(">>> InstSelectSort:")
	slice := []int{14, 33, 10, 27, 19, 35, 42, 44, 18}
	fmt.Printf("    slice = %v\n", slice)

	n := SelectSort(slice)
	fmt.Printf("    n = %d, slice = %v\n", n, slice)
}

/*
  14, 33, 10, 27, 19, 35, 42, 44, 18

  10,| 33, 14, 27, 19, 35, 42, 44, 18
  10, 14,| 33, 27, 19, 35, 42, 44, 18
  10, 14, 18,| 27, 19, 35, 42, 44, 33
  10, 14, 18, 19,| 27, 35, 42, 44, 33
  10, 14, 18, 19, 27,| 35, 42, 44, 33
  10, 14, 18, 19, 27, 33,| 42, 44, 35
  10, 14, 18, 19, 27, 33, 35,| 44, 42
  10, 14, 18, 19, 27, 33, 35, 42,| 44
*/

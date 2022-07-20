package main

import (
	"fmt"
)

type Ordered interface {
	~float64 | ~int64 | ~string
}

func quicksort[T Ordered](data []T, low, high int) {
	if low < high {
		var idx = partition(data, low, high)
		fmt.Printf("    output: %v, pivotIndex: %d\n", data, idx)

		if low < idx {
			quicksort(data, low, idx)
		}
		if idx+1 < high {
			quicksort(data, idx+1, high)
		}
	}
}

func partition[T Ordered](data []T, low, high int) int {
	fmt.Printf("~~~ partition: %v, %d - %d\n", data, low, high)
	// Pick a lowest bound element as a pivot value
	var pivot = data[low]
	var i, j = low, high

	for i < j {
		for data[i] <= pivot && i < high {
			i++
		}
		for data[j] > pivot && j > low {
			j--
		}
		if i < j {
			fmt.Printf("    swap %d@%d, %d@%d, ", data[i], i, data[j], j)
			data[i], data[j] = data[j], data[i]
			fmt.Println(data)
		}
	}

	if j > low {
		fmt.Printf("    swap %d@%d, %d@%d, ", data[low], low, data[j], j)
		data[low], data[j] = data[j], pivot
		fmt.Println(data)
	}
	return j
}

func main() {
	ints := []int64{7, 2, 1, 8, 6, 3, 5, 4}
	fmt.Println(">>>", ints)
	quicksort[int64](ints, 0, len(ints)-1)
	fmt.Println(ints)

	//	numbers := []float64{3.5, -2.4, 12.8, 9.1, 4.5, 1.0}
	//	fmt.Println(">>>", numbers)
	//	quicksort[float64](numbers, 0, len(numbers)-1)
	//	fmt.Println(numbers)

	//	names := []string{"Zachary", "John", "Moe", "Jim", "Robert"}
	//	fmt.Println(">>>", names)
	//	quicksort[string](names, 0, len(names)-1)
	//	fmt.Println(names)
}

package main

import (
	"fmt"
)

type Ordered interface {
	~float64 | ~int64 | ~string
}

func quicksort[T Ordered](data []T, low, high int) {
	if low < high {
		var pivot = partition(data, low, high)
		quicksort(data, low, pivot)
		quicksort(data, pivot+1, high)
	}
}

func partition[T Ordered](data []T, low, high int) int {
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
			b := fmt.Sprintf("%v", data)
			data[i], data[j] = data[j], data[i]
			fmt.Printf("~~~ 1: %v -> %v\n", b, data)
		}
	}

	b := fmt.Sprintf("%v", data)
	data[low], data[j] = data[j], pivot
	fmt.Printf("~~~ 2: %v -> %v\n", b, data)
	return j
}

func main() {
	numbers := []float64{3.5, -2.4, 12.8, 9.1, 4.5, 1.0}
	fmt.Println(">>>", numbers)
	quicksort[float64](numbers, 0, len(numbers)-1)
	fmt.Println(numbers)

	names := []string{"Zachary", "John", "Moe", "Jim", "Robert"}
	fmt.Println(">>>", names)
	quicksort[string](names, 0, len(names)-1)
	fmt.Println(names)
}

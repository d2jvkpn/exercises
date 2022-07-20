package main

import (
	"fmt"
)

type Ordered interface {
	~float64 | ~int | ~string
}

func bubblesort[T Ordered](data []T) {
	n := len(data)
	for i := 0; i < n-1; i++ {
		for j := 0; j < n-1-i; j++ {
			if data[j] > data[j+1] {
				data[j], data[j+1] = data[j+1], data[j]
			}
		}
	}
}
func main() {
	numbers := []float64{3.5, -2.4, 12.8, 9.1}
	bubblesort[float64](numbers)
	fmt.Println(numbers)

	names := []string{"Zachary", "John", "Moe", "Jim", "Robert"}
	bubblesort[string](names)
	fmt.Println(names)
}

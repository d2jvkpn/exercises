// author: ChatGPT
package go_impls

import "fmt"

func heapify(arr []int, n, i int) {
	// fmt.Printf("--> heapify: %v, %d, %d\n", arr, n, i)
	largest := i
	left, right := 2*i+1, 2*i+2

	if left < n && arr[left] > arr[largest] {
		largest = left
	}

	if right < n && arr[right] > arr[largest] {
		largest = right
	}

	if largest != i {
		// fmt.Printf("    swap: arr[%d]=%d, arr[%d]=%d\n", largest, arr[largest], i, arr[i])
		arr[i], arr[largest] = arr[largest], arr[i]
		heapify(arr, n, largest)
	}
}

func heapSort(arr []int) {
	n := len(arr)

	// Build max-heap
	for i := n/2 - 1; i >= 0; i-- {
		heapify(arr, n, i)
	}

	fmt.Println("~~~ data:", arr)

	// Extract elements from the heap
	for i := n - 1; i > 0; i-- {
		arr[0], arr[i] = arr[i], arr[0]
		heapify(arr, i, 0)
	}
}

func main() {
	data := []int{12, 11, 13, 5, 6, 7, 20, 6, 7, 3}
	fmt.Println("Unsorted array:", data)

	heapSort(data)
	fmt.Println("Sorted array:", data)
}

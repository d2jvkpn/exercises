package explore

func partition(arr []int, low, high int) int {
	index := low - 1

	pivotElement := arr[high]
	for i := low; i < high; i++ {
		if arr[i] <= pivotElement {
			index += 1
			arr[index], arr[i] = arr[i], arr[index]
		}
	}
	arr[index+1], arr[high] = arr[high], arr[index+1]
	return index + 1
}

func quicksortRange(arr []int, low, high int) {
	if len(arr) <= 1 {
		return
	}

	if low < high {
		pivot := partition(arr, low, high)
		quicksortRange(arr, low, pivot-1)
		quicksortRange(arr, pivot+1, high)
	}
}

// Quicksort Sorts the entire array
func Quicksort(arr []int) []int {
	quicksortRange(arr, 0, len(arr)-1)
	return arr
}

/*
  14, 33, 10, 27, 19, 35, 42, 44, =18

  13, 10, 33, 23, 19, 35, 42, 44, 18
  13, 10, 18, =23, 19, 35, 42, 44, 33

  19, 35, 42, 44, =33
  19, 33, =42, 44, 35

  44, 45

*/

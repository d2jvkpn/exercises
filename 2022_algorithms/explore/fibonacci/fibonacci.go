package fibonacci

import (
// "fmt"
)

func Fib(n int, c *int) int {
	*c += 1

	if n <= 2 {
		return 1
	}
	return Fib(n-1, c) + Fib(n-2, c)
}

func FibTailRecursion(n, a, b int, c *int) int {
	*c += 1

	if n == 0 {
		return a
	}
	if n == 1 {
		return b
	}
	return FibTailRecursion(n-1, b, a+b, c)
}

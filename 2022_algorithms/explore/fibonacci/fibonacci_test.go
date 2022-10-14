package fibonacci

import (
	"fmt"
	"testing"
)

func TestFib(t *testing.T) {
	for i := 1; i <= 10; i++ {
		c := new(int)
		fmt.Printf(">>> Fid(%d)=%d, count=%d\n", i, Fib(i, c), *c)
	}

	for i := 1; i <= 10; i++ {
		c := new(int)
		fmt.Printf(">>> Fid(%d)=%d, count=%d\n", i, FibTailRecursion(i, 0, 1, c), *c)
	}
}

// go test -run none -bench ^BenchmarkFib$ -benchmem -count 5
func BenchmarkFib(b *testing.B) {
	for i := 0; i < b.N; i++ {
		c := new(int)
		Fib(10, c)
	}
}

// go test -run none -bench ^BenchmarkFibTailRecursion$ -benchmem -count 5
func BenchmarkFibTailRecursion(b *testing.B) {
	for i := 0; i < b.N; i++ {
		c := new(int)
		FibTailRecursion(10, 0, 1, c)
	}
}

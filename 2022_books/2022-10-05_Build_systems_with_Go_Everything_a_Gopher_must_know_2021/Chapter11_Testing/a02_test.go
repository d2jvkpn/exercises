package main

import (
	// "fmt"
	"testing"
)

func Sum(n int64) int64 {
	var result int64 = 0

	for i := 0; int64(i) < n; i++ {
		result += int64(i)
	}

	return result
}

func BenchmarkSum(b *testing.B) {
	// fmt.Println("~~~ b.N:", b.N)

	for i := 0; i < b.N; i++ {
		Sum(1000000)
	}
}

func BenchmarkSumParallel(b *testing.B) {
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			BenchmarkSum(b)
		}
	})
}

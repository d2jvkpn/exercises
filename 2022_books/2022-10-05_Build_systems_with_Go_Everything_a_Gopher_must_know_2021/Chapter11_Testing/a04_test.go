package main

import (
	"testing"
)

func BenchmarkGraph(b *testing.B) {
	for i := 0; i < b.N; i++ {
		BuildGraph(100, 20000)
	}
}

package main

import (
	"math/rand"
	"testing"
	"time"
)

//func init() {
//	rand.Seed(time.Now().UnixNano())
//}

func BuildGraph(vertices int, edges int) [][]int {
	rng := rand.New(rand.NewSource(time.Now().UnixNano()))
	graph := make([][]int, vertices)

	for i := 0; i < len(graph); i++ {
		graph[i] = make([]int, 0, 1)
	}

	for i := 0; i < edges; i++ {
		// from := rand.Intn(vertices)
		// to := rand.Intn(vertices)
		from := rng.Intn(vertices)
		to := rng.Intn(vertices)
		graph[from] = append(graph[from], to)
	}

	return graph
}

func BenchmarkGraph(b *testing.B) {
	for i := 0; i < b.N; i++ {
		BuildGraph(100, 20000)
	}
}

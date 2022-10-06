package main

import (
	"fmt"
	"math/rand"
	"time"
)

//func init() {
//	rand.Seed(time.Now().UnixNano())
//}

// Periods funtiono doc
func Periods(year int) string {
	switch {
	case year < -3000:
		return "Copper Age"
	case year < -2000:
		return "Bronze Age"
	case year < -1000:
		return "Iron Age"
	case year < 0:
		return "Classic Age"
	case year < 476:
		return "Roman Age"
	case year < 1492:
		return "Middle Age"
	case year < 1800:
		return "Modern Age"
	default:
		return "unknown"
	}
}

func ExamplePeriods() {
	desc := Periods(1799)
	fmt.Println(desc)
	// Output:
	// Modern Age
}

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

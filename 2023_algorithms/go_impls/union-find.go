package go_impls

import (
// "fmt"
)

type UnionFind struct {
	Parent map[int]int
	Rank   map[int]int
}

func NewUnionFind(n int) *UnionFind {
	parent := make(map[int]int, n)
	rank := make(map[int]int, n)

	for i := 1; i < n+1; i++ {
		parent[i] = i
		rank[i] = 0
	}

	return &UnionFind{Parent: parent, Rank: rank}
}

func (uf *UnionFind) Find(val int) int {
	if val == 0 {
		return 0
	}

	p := uf.Parent[val]

	for p != uf.Parent[p] {
		uf.Parent[p] = uf.Parent[uf.Parent[p]]
		p = uf.Parent[p]
	}

	return p
}

func (uf *UnionFind) Union(n1, n2 int) bool {
	if n1 == 0 || n2 == 0 {
		return false
	}

	p1, p2 := uf.Find(n1), uf.Find(n2)
	if p1 == p2 {
		return false
	}

	if uf.Rank[p1] > uf.Rank[p2] {
		uf.Parent[p2] = p1
	} else if uf.Rank[p1] < uf.Rank[p2] {
		uf.Parent[p1] = p2
	} else {
		uf.Parent[p1] = p2
		uf.Rank[p2] += 1
	}

	return true
}

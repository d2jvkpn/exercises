package main

import (
	"fmt"
	"testing"
)

func TestFaSP(t *testing.T) {
	list := NewLinkedList[int]()

	list.Push(1).Push(2).Push(3).Push(2).Push(4).Push(5)
	fmt.Printf("~~~ list: %+v, size: %d\n", list.Header, list.Size)

	n1, n4 := list.Get(1), list.Get(4)
	fmt.Printf("~~~ n1: %+v, n4: %+v\n", n1, n4)
	n4.Next = n1

	node, hasCycle := list.HasCycle()
	if !hasCycle {
		t.Fatalf("don't has cycle")
	}
	fmt.Printf("~~~ node: %+v\n", node)

	cStart := list.CycleStart()
	if cStart == nil {
		t.Fatalf("don't has cycle start")
	}
	if cStart.Value != 2 {
		t.Fatalf("incorrect cycle start: %d, expected: 2\n", cStart.Value)
	}
}

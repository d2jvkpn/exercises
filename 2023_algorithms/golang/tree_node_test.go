package goimpls

import (
	"fmt"
	"math/rand"
	"testing"
	"time"
)

func TestTreeNode(t *testing.T) {
	arr := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	length := len(arr)
	rand.Seed(time.Now().UnixNano())
	rand.Shuffle(len(arr), func(i, j int) { arr[i], arr[j] = arr[j], arr[i] })
	fmt.Println("~~~ arr:", arr)

	// node := NewNode(6).Push(3).Push(9).Push(2).Push(4).Push(1).Push(5).Push(7).Push(10).Push(8)
	node := NewTreeNode(arr[0])
	for _, v := range arr[1:] {
		node.Push(v)
	}

	tree1 := &BinaryTree{Header: node}
	fmt.Println(">>> tree1.Header:", tree1.Header)
	fmt.Println("    height:", tree1.Height())

	if v := tree1.Count(); v != length {
		t.Fatalf("!!! count=%d, expected=%d\n", v, length)
	}

	if v := tree1.Find(3); v == nil {
		t.Fatalf("!!! can't find 3")
	}

	if v := tree1.Find(11); v != nil {
		t.Fatalf("!!! unexpected find 11")
	}

	yes := tree1.Delete(12)
	if yes {
		t.Fatalf("!!! can't delete 12 as it's not exists")
	}

	fmt.Println(">>> delete 2", tree1.Delete(2))
	if v := tree1.Count(); v != length-1 {
		t.Fatalf("!!! count=%d, expected=%d\n", v, length-1)
	}

	value := tree1.Header.Value
	fmt.Println(">>> delete header", value, tree1.Delete(value))
	if tree1.Header.Value == value {
		t.Fatalf("!!! header unchanged after delete")
	}
	if v := tree1.Count(); v != length-2 {
		t.Fatalf("!!! count=%d, expected=%d\n", v, length-2)
	}
}

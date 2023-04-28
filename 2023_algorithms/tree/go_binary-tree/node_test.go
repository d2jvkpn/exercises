package main

import (
	"fmt"
	"testing"
)

func TestNode(t *testing.T) {
	node := NewNode(6).Push(3).Push(9).Push(2).Push(4).Push(1).Push(5).Push(7).Push(10).Push(8)
	fmt.Println("node:", node)
	fmt.Println("count:", node.Count())
	fmt.Println("find(3):", node.Find(3))
	fmt.Println("find(3):", node.Find(11))

	tree := &BinaryTree{Header: node}
	fmt.Println("tree.Header:", tree.Header)
	tree.Delete(2)
	fmt.Println("count:", tree.Header.Count())
	tree.Delete(6)
	fmt.Println("tree.Header:", tree.Header)
	fmt.Println("count:", tree.Header.Count())

	tree2 := &BinaryTree{Header: NewNode(6)}
	tree2.Delete(6)
	fmt.Println("tree2.Header:", tree2.Header)
}

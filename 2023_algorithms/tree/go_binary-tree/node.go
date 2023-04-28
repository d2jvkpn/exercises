package main

import (
// "fmt"
)

type Node struct {
	Value int
	Left  *Node
	Right *Node
}

func NewNode(value int) *Node {
	return &Node{Value: value}
}

func (node *Node) Push(value int) *Node {
	nd := NewNode(value)

	switch {
	case value < node.Value && node.Left == nil:
		node.Left = nd
	case value < node.Value && node.Left != nil:
		node.Left.Push(value)
	case value > node.Value && node.Right == nil:
		node.Right = nd
	case value > node.Value && node.Right != nil:
		node.Right.Push(value)
	default:
		return node
	}

	return node
}

func (node *Node) Find(value int) *Node {
	switch {
	case node == nil:
		return nil
	case node.Value == value:
		return node
	case value < node.Value:
		return node.Left.Find(value)
	case value > node.Value && node.Right != nil:
		return node.Right.Find(value)
	default:
		return nil
	}
}

func (node *Node) FindWithParent(value int) (*Node, *Node) {
	switch {
	case node == nil:
		return nil, nil
	case node.Value == value:
		return nil, node
	case value < node.Value:
		if node.Left == nil {
			return nil, nil
		} else if value == node.Left.Value {
			return node, node.Left
		} else {
			return node.Left.FindWithParent(value)
		}
	case value < node.Value:
		if node.Right == nil {
			return nil, nil
		} else if value == node.Right.Value {
			return node, node.Right
		} else {
			return node.Right.FindWithParent(value)
		}
	default:
		return nil, nil
	}
}

func (node *Node) successor() (int, bool) {
	var (
		val int
		nd  *Node
	)

	switch {
	case node.Left == nil && node.Right == nil:
		return 0, false
	case node.Left != nil && node.Right == nil:
		nd = node.Left
		val = nd.Value
		node.Left, node.Right = nd.Left, nd.Right
		// gc nd
		return val, true
	case node.Left == nil && node.Right != nil:
		nd = node.Right
		val = nd.Value
		node.Left, node.Right = nd.Left, nd.Right
		// gc nd
		return val, true
	}

	nd = node.Left
	for nd.Right != nil {
		nd = nd.Right
	}
	val = node.Value

	if nd.Left != nil {
		nd.Left, nd.Right = nd.Left.Left, nd.Left.Right
	}

	return val, true
}

func (node *Node) sum(count *int) {
	if node == nil {
		return
	}
	*count += 1

	node.Left.sum(count)
	node.Right.sum(count)
}

func (node *Node) Sum() int {
	count := new(int)
	node.sum(count)

	return *count

}

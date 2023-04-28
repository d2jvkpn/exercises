package binary_tree

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
	switch {
	case value < node.Value && node.Left == nil:
		node.Left = NewNode(value)
	case value < node.Value && node.Left != nil:
		node.Left.Push(value)
	case value > node.Value && node.Right == nil:
		node.Right = NewNode(value)
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
	case value > node.Value:
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

func (node *Node) succeed() (value int, ok bool) {
	var parent, current *Node

	/*
		t := 0
		defer func() {
			println("!!! t:", t)
		}()
	*/

	switch {
	case node.Left == nil && node.Right == nil:
		// t = 1
		return 0, false
	case node.Left != nil && node.Right == nil:
		// t = 2
		current = node.Left
		node.Value = current.Value
		node.Left, node.Right = current.Left, current.Right
		return node.Value, true
	case node.Left == nil && node.Right != nil:
		// t = 3
		current = node.Right
		node.Value = current.Value
		node.Left, node.Right = current.Left, current.Right
		return node.Value, true
	}

	current = node.Right
	switch {
	case current.Left == nil && current.Right == nil:
		// t = 4
		node.Value = current.Value
		node.Right = nil
	case current.Left == nil && current.Right != nil:
		// t = 5
		node.Value = current.Value
		node.Right = current.Right
	default: // node.Left != nil
		parent = current
		current = parent.Left // not nil
		for current.Left != nil {
			parent = current
			current = current.Left
		}

		node.Value = current.Value
		if current.Right != nil {
			// t = 6
			parent.Left = current.Right
		} else {
			// t = 7
			parent.Left = nil
		}
	}

	return node.Value, true
}

func (node *Node) count(count *int) {
	if node == nil {
		return
	}
	*count += 1

	node.Left.count(count)
	node.Right.count(count)
}

func (node *Node) Count() int {
	count := new(int)
	node.count(count)

	return *count
}

func (node *Node) Levels() (levels int) {
	switch {
	case node == nil:
		return 0
	case node.Left == nil && node.Right == nil:
		return 1
	case node.Left != nil && node.Right == nil:
		levels = node.Left.Levels()
		return levels + 1
	case node.Left == nil && node.Right != nil:
		levels = node.Right.Levels()
		return levels + 1
	default:
		left, right := node.Left.Levels(), node.Right.Levels()
		if left > right {
			levels = left
		} else {
			levels = right
		}
		return levels + 1
	}
}

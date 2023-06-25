package goimpls

import (
	"fmt"
)

type TreeNode struct {
	Value int
	Left  *TreeNode
	Right *TreeNode
}

func NewTreeNode(value int) *TreeNode {
	return &TreeNode{Value: value}
}

func (node *TreeNode) Push(value int) *TreeNode {
	switch {
	case value < node.Value && node.Left == nil:
		node.Left = NewTreeNode(value)
	case value < node.Value && node.Left != nil:
		node.Left.Push(value)
	case value > node.Value && node.Right == nil:
		node.Right = NewTreeNode(value)
	case value > node.Value && node.Right != nil:
		node.Right.Push(value)
	default:
		return node
	}

	return node
}

func (node *TreeNode) Find(value int) *TreeNode {
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

func (node *TreeNode) FindWithParent(value int) (*TreeNode, *TreeNode) {
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

func (node *TreeNode) succeed() (value int, ok bool) {
	var parent, current *TreeNode

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
	default: // node.Left != nil && node.Right != nil
	}

	current = node.Right
	switch {
	case current.Left == nil && current.Right == nil:
		// t = 4
		node.Value = current.Value
		node.Right = nil
		return node.Value, true
	case current.Left == nil && current.Right != nil:
		// t = 5
		node.Value = current.Value
		node.Right = current.Right
		return node.Value, true
	default: // current.Left != nil
	}

	parent = current      // node.Right
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

	return node.Value, true
}

func (node *TreeNode) count(count *int) {
	if node == nil {
		return
	}
	*count += 1

	node.Left.count(count)
	node.Right.count(count)
}

func (node *TreeNode) Count() int {
	count := new(int)
	node.count(count)

	return *count
}

func (node *TreeNode) Height() (levels int) {
	switch {
	case node == nil:
		return 0
	case node.Left == nil && node.Right == nil:
		return 1
	case node.Left != nil && node.Right == nil:
		levels = node.Left.Height()
	case node.Left == nil && node.Right != nil:
		levels = node.Right.Height()
	default:
		left, right := node.Left.Height(), node.Right.Height()
		if left > right {
			levels = left
		} else {
			levels = right
		}
	}

	return levels + 1
}

// left, parent, right
func (node *TreeNode) InorderTraversal() {
	if node == nil {
		return
	}

	node.Left.InorderTraversal()
	fmt.Println("==>", node.Value)
	node.Right.InorderTraversal()
}

// parent, left, right
func (node *TreeNode) PreorderTraversal() {
	fmt.Println("==>", node.Value)
	node.Left.InorderTraversal()
	node.Right.InorderTraversal()
}

// left, right, root
func (node *TreeNode) PostorderTraversal() {
	node.Left.InorderTraversal()
	node.Right.InorderTraversal()
	fmt.Println("==>", node.Value)
}

package binary_tree

import (
// "fmt"
)

type BinaryTree struct {
	Header *Node
}

func NewBinaryTree() BinaryTree {
	return BinaryTree{Header: nil}
}

func (tree *BinaryTree) Push(value int) *Node {
	if tree.Header == nil {
		tree.Header = NewNode(value)
		return tree.Header
	}

	return tree.Header.Push(value)
}

func (tree *BinaryTree) Find(value int) *Node {
	return tree.Header.Find(value)
}

func (tree *BinaryTree) Delete(value int) bool {
	parent, target := tree.Header.FindWithParent(value)
	if target == nil {
		return false
	}

	_, ok := target.succeed()
	if !ok {
		if parent == nil {
			tree.Header = nil
		} else if parent.Left == target {
			parent.Left = nil
		} else {
			parent.Right = nil
		}
	}

	return true
}

func (tree *BinaryTree) Count() int {
	return tree.Header.Count()
}

func (tree *BinaryTree) Height() int {
	return tree.Header.Height()
}

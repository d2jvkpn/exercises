package go_impls

import (
// "fmt"
)

type BinaryTree struct {
	Header *TreeNode
}

func NewBinaryTree() BinaryTree {
	return BinaryTree{Header: nil}
}

func (tree *BinaryTree) Push(value int) *TreeNode {
	if tree.Header == nil {
		tree.Header = NewTreeNode(value)
		return tree.Header
	}

	return tree.Header.Push(value)
}

func (tree *BinaryTree) Find(value int) *TreeNode {
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

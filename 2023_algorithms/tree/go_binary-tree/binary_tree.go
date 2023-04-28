package main

import (
// "fmt"
)

type BinaryTree struct {
	Header *Node
}

func NewBinaryTree() BinaryTree {
	return BinaryTree{
		Header: nil,
	}
}

func (tree *BinaryTree) Delete(val int) bool {
	parent, target := tree.Header.FindWithParent(val)
	if target == nil {
		return false
	}

	val, ok := target.succeed()
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

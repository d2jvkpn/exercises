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

	switch {
	case target == nil: // parent == nil
		return false
	case parent == nil: // target != nil, target == tree.Header
		tree.Header = nil
		return true
	}

	if val, ok := target.successor(); !ok { // target has no children
		if parent.Left == target {
			parent.Left = nil
		} else {
			parent.Right = nil
		}
	} else {
		target.Value = val
	}

	return true
}

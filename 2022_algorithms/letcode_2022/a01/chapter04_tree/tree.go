package main

import (
	// "fmt"
	"math"
)

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func MaxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	if root.Left == nil && root.Right == nil {
		return 1
	}

	lHeight := MaxDepth(root.Left)
	rHeight := MaxDepth(root.Right)

	if lHeight >= rHeight {
		return lHeight + 1
	} else {
		return rHeight + 1
	}
}

func isSymmetricTree(p *TreeNode, q *TreeNode) bool {
	if p == q {
		return true
	}
	if p == nil || q == nil || p.Val != q.Val {
		return false
	}
	return isSymmetricTree(p.Left, q.Right) && isSymmetricTree(p.Right, q.Left)
}

func isSymmetric(root *algorithms.TreeNode) bool {
	if root == nil {
		return true
	}
	return isSymmetricTree(root.Left, root.Right)
}

func validBST(node *TreeNode, lower, upper int) bool {
	if node == nil {
		return true
	}

	if node.Val <= lower || node.Val >= upper {
		return false
	}

	return validBST(node.Left, lower, node.Val) && validBST(node.Right, node.Val, upper)
}

func IsValidBST(root *TreeNode) bool {
	return validBST(root, math.MinInt64, math.MaxInt64)
}

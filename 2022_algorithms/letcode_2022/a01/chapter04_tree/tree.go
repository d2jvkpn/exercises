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
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil || p.Val != q.Val {
		return false
	}
	return isSymmetricTree(p.Left, q.Right) && isSymmetricTree(p.Right, q.Left)
}

func isSymmetric(root *TreeNode) bool {
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

func levelOrderHelper(node *TreeNode) [][]int {
	if node.Left == nil && node.Right == nil {
		return nil
	}

	if node.Left != nil && node.Right == nil {
		return [][]int{{node.Left.Val}}
	}
	if node.Left == nil && node.Right != nil {
		return [][]int{{node.Right.Val}}
	}

	p := levelOrderHelper(node.Left)
	q := levelOrderHelper(node.Right)
	res := make([][]int, len(p)+len(q)+1)
	res = append(res, []int{node.Left.Val, node.Right.Val})

	for i, j := 0, 0; i < len(p) || j < len(q); {
		if i < len(p) {
			res = append(res, p[i])
			i++
		}

		if j < len(q) {
			res = append(res, q[j])
			j++
		}
	}

	return res
}

func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	res := [][]int{{root.Val}}
	return append(res, levelOrderHelper(root)...)
}

func sortedArrayToBST(nums []int) *TreeNode {
	if len(nums) == 0 {
		return nil
	}

	mid := len(nums) / 2

	return &TreeNode{
		Val:   nums[mid],
		Left:  sortedArrayToBST(nums[:mid]),
		Right: sortedArrayToBST(nums[mid+1:]),
	}
}

package main

import (
// "fmt"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversal(root *TreeNode) []int {
	result := make([]int, 0)

	if root == nil {
		return result
	}

	if root.Left != nil {
		result = append(result, inorderTraversal(root.Left)...)
	}
	result = append(result, root.Val)
	if root.Right != nil {
		result = append(result, inorderTraversal(root.Right)...)
	}

	return result
}

func inorderTraversal2(root *TreeNode) []int {
	result := make([]*TreeNode, 0)
	values := make([]int, 0)
	curr := root

	for {
		if curr != nil {
			result = append(result, curr)
			curr = curr.Left
		} else if len(result) != 0 {
			n := len(result) - 1
			curr = result[n]
			values = append(values, curr.Val)
			curr = curr.Right
			result = result[:n]
		} else {
			break
		}
	}

	return values
}

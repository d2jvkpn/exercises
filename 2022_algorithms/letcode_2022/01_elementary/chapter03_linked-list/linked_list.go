package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func NewListNode(arr []int) *ListNode {
	if len(arr) == 0 {
		return nil
	}

	node := &ListNode{
		Val:  arr[0],
		Next: nil,
	}

	last := node
	for _, v := range arr[1:] {
		curr := &ListNode{
			Val:  v,
			Next: nil,
		}

		last.Next = curr
		last = curr
	}

	return node
}

func PrintListNode(head *ListNode) {
	curr := head
	for {
		if curr == nil {
			break
		}
		fmt.Print(curr.Val, " ")
		curr = curr.Next
	}

	fmt.Println()
}

func DeleteNode(node *ListNode) {
	next := node.Next
	node.Val = next.Val
	node.Next = next.Next
}

func RemoveNthFromEnd(head *ListNode, n int) *ListNode {
	s, curr := 0, head
	for {
		if curr == nil {
			break
		}
		s++
		curr = curr.Next
	}
	if n > s {
		return nil
	} else if n == s {
		return head.Next
	}

	curr = head
	for i := 1; i <= s; i++ {
		if s-i == n {
			curr.Next = curr.Next.Next
			// return curr.Next
			break
		}
		curr = curr.Next
	}

	return head
}

func Reverse(head *ListNode) *ListNode {
	var (
		prev *ListNode = nil
		curr *ListNode = head
	)

	for curr != nil {
		tmp, next := curr, curr.Next
		tmp.Next = prev
		prev = tmp
		curr = next
	}

	return prev
}

func HasCycle(head *ListNode) bool {
	if head == nil {
		return false
	}

	mp := make(map[*ListNode]bool)

	for curr := head; curr.Next != nil; curr = curr.Next {
		if mp[curr] {
			return true
		}
		mp[curr] = true
	}

	return false
}

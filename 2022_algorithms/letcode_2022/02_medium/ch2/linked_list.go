package main

import (
	"fmt"
)

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func NewListNode(values []int) *ListNode {
	if len(values) == 0 {
		return nil
	}

	head := &ListNode{Val: values[0]}
	tail := head

	for _, v := range values[1:] {
		node := &ListNode{Val: v}
		tail.Next = node
		tail = node
	}

	return head
}

func (item *ListNode) String() string {
	values := make([]int, 0)

	for curr := item; curr != nil; curr = curr.Next {
		values = append(values, curr.Val)
	}

	return fmt.Sprintf("%v", values)
}

func AddTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var (
		count, s1, s2, sum int
		curr, node         *ListNode
	)

	count = 0
	for curr = l1; curr != nil; curr = curr.Next {
		//		s1 *= 10
		//		s1 += curr.Val
		val := curr.Val
		for i := 0; i < count; i++ {
			val *= 10
		}
		s1 += val
		count++
	}

	count = 0
	for curr = l2; curr != nil; curr = curr.Next {
		//		s2 *= 10
		//		s2 += curr.Val
		val := curr.Val
		for i := 0; i < count; i++ {
			val *= 10
		}
		s2 += val
		count++
	}

	sum = s1 + s2
	// fmt.Println("~~~", sum, s1, s2)

	if sum == 0 {
		return &ListNode{
			Val:  0,
			Next: nil,
		}
	}

	for curr = nil; sum > 0; sum /= 10 {
		tmp := &ListNode{
			Val:  sum % 10,
			Next: nil,
		}
		if curr == nil {
			node = tmp
		} else {
			curr.Next = tmp
		}
		curr = tmp
	}

	return node
}

func OddEvenList(head *ListNode) *ListNode {
	var (
		idx, length int
		curr        *ListNode
	)

	for curr = head; curr != nil; curr = curr.Next {
		length += 1
	}
	// fmt.Println("~~~ length:", length)

	for curr = head; curr != nil && idx < length/2; curr, idx = curr.Next, idx+1 {
		prev := curr.Next
		for i := 0; i < idx; i++ {
			prev = prev.Next
		}

		// fmt.Println("   ", curr.Next.Val, prev.Next.Val)
		target := prev.Next
		if target == nil {
			break
		}
		prev.Next = target.Next
		target.Next = curr.Next
		curr.Next = target
	}

	return head
}

func GetIntersectionNode(headA, headB *ListNode) *ListNode {
	mp := make(map[*ListNode]bool)

	for curr := headA; curr != nil; curr = curr.Next {
		mp[curr] = true
	}

	for ; headB != nil; headB = headB.Next {
		if !mp[headB] {
			continue
		}
		found := true

		for curr := headB; curr != nil; curr = curr.Next {
			if !mp[curr] {
				found = false
				break
			}
		}

		if found {
			break
		}
	}

	if mp[headB] {
		return headB
	} else {
		return nil
	}
}

package main

import (
	"fmt"
	"testing"
)

func TestRemoveNthFromEnd(t *testing.T) {
	head := NewListNode([]int{1, 2, 3, 4, 5})
	fmt.Println(head)

	PrintListNode(head)
	out := RemoveNthFromEnd(head, 2)
	fmt.Println(">>>", out)
	PrintListNode(head)

	head = NewListNode([]int{1, 2})
	out = RemoveNthFromEnd(head, 2)
	fmt.Println(">>>", out)
	PrintListNode(head)
}

func TestReverse(t *testing.T) {
	head := NewListNode([]int{1, 2, 3, 4, 5})
	PrintListNode(head)

	PrintListNode(Reverse(head))
}

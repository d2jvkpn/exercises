package main

import (
	"fmt"
	"testing"
)

func TestListNode(t *testing.T) {
	l1 := NewListNode([]int{2, 4, 3})
	l2 := NewListNode([]int{5, 6, 4})
	fmt.Println(l1, l2)

	fmt.Println(AddTwoNumbers(l1, l2))

	l1 = NewListNode([]int{
		1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		1,
	})
	l2 = NewListNode([]int{5, 6, 4})
	fmt.Println(AddTwoNumbers(l1, l2))
}

func TestOddEvenList(t *testing.T) {
	list := NewListNode([]int{2, 1, 3, 5, 6, 4, 7})
	fmt.Println(">>>", list)
	fmt.Println(OddEvenList(list))

	list = NewListNode([]int{1, 2, 3, 4, 5, 6, 7, 8})
	fmt.Println(">>>", list)
	fmt.Println(OddEvenList(list))
}

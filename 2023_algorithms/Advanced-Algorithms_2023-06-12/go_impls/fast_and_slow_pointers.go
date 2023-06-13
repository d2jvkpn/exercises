package main

import (
	// "fmt"
	. "golang.org/x/exp/constraints"
)

type LinkedList[T Ordered] struct {
	Header *Node[T]
	Size   uint
}

type Node[T Ordered] struct {
	Value T
	Next  *Node[T]
}

func NewNode[T Ordered](value T) *Node[T] {
	return &Node[T]{Value: value, Next: nil}
}

func NewLinkedList[T Ordered]() *LinkedList[T] {
	return &LinkedList[T]{Header: nil, Size: 0}
}

func (list *LinkedList[T]) Push(value T) *LinkedList[T] {
	node := list.Header

	if node == nil {
		list.Size += 1
		list.Header = NewNode[T](value)
		return list
	}

	for node != nil {
		if node.Value == value {
			return list
		}

		if node.Next == nil {
			node.Next = NewNode[T](value)
			break
		}

		node = node.Next
	}

	list.Size += 1
	return list
}

func (list *LinkedList[T]) Get(idx int) *Node[T] {
	node := list.Header

	for i := 0; ; i++ {
		if i == idx {
			return node
		}
		node = node.Next
	}
}

func (list *LinkedList[T]) HasCycle() (*Node[T], bool) {
	slow, fast := list.Header, list.Header

	for {
		if fast == nil {
			return nil, false
		}

		slow = slow.Next
		if fast = fast.Next; fast == nil {
			return nil, false
		}
		fast = fast.Next

		if fast.Value == slow.Value {
			return fast, true
		}
	}
}

func (list *LinkedList[T]) CycleStart() *Node[T] {
	fast, hasCycle := list.HasCycle()
	if !hasCycle {
		return nil
	}

	slow := list.Header

	for {
		if fast.Value == slow.Value {
			return slow
		}

		slow, fast = slow.Next, fast.Next
	}
}

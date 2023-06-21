package go_impls

import (
	// "fmt"
	. "golang.org/x/exp/constraints"
)

type LinkedList[T Ordered] struct {
	Header *LinkedNode[T]
	Size   uint
}

func NewLinkedList[T Ordered]() *LinkedList[T] {
	return &LinkedList[T]{Header: nil, Size: 0}
}

func (list *LinkedList[T]) Push(value T) *LinkedList[T] {
	node := list.Header

	if node == nil {
		list.Size += 1
		list.Header = NewLinkedNode[T](value)
		return list
	}

	for node != nil {
		if node.Value == value {
			return list
		}

		if node.Next == nil {
			node.Next = NewLinkedNode[T](value)
			break
		}

		node = node.Next
	}

	list.Size += 1
	return list
}

func (list *LinkedList[T]) Get(idx int) *LinkedNode[T] {
	node := list.Header

	for i := 0; ; i++ {
		if i == idx {
			return node
		}
		node = node.Next
	}
}

func (list *LinkedList[T]) HasCycle() (*LinkedNode[T], bool) {
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

func (list *LinkedList[T]) CycleStart() *LinkedNode[T] {
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

package goimpls

import (
	// "fmt"
	. "golang.org/x/exp/constraints"
)

type LinkedNode[T Ordered] struct {
	Value T
	Next  *LinkedNode[T]
}

func NewLinkedNode[T Ordered](value T) *LinkedNode[T] {
	return &LinkedNode[T]{Value: value, Next: nil}
}

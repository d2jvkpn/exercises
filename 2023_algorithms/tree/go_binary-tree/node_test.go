package main

import (
	"fmt"
	"testing"
)

func TestNode(t *testing.T) {
	node := NewNode(6).Push(3).Push(9).Push(2).Push(4).Push(1).Push(5).Push(7).Push(10).Push(8)
	fmt.Printf("%+v\n", node)
}

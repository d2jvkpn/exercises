package main

import (
	"fmt"
)

type Node struct {
	Value string
	Next  []*Node
}

func NewNode(value string) *Node {
	return &Node{Value: value}
}

func (node *Node) count(vm, em map[string]bool) {
	vm[node.Value] = true

	for i := range node.Next {
		em[fmt.Sprintf("%s-%s", node.Value, node.Next[i].Value)] = true
		node.Next[i].count(vm, em)
	}
}

func (node *Node) String() string {
	return node.Value
}

func (node *Node) Count() (int, int) {
	vm, em := make(map[string]bool), make(map[string]bool)
	node.count(vm, em)

	return len(vm), len(em)
}

func main() {
	a := NewNode("A")
	b := NewNode("B")
	c := NewNode("C")
	d := NewNode("D")
	e := NewNode("E")
	f := NewNode("F")
	g := NewNode("G")

	a.Next = []*Node{b, c}
	b.Next = []*Node{d, e}
	c.Next = []*Node{f}
	d.Next = []*Node{f}
	e.Next = []*Node{f}
	f.Next = []*Node{g}

	v1, e1 := a.Count()
	fmt.Printf("~~~ v1=%d, e1=%d\n", v1, e1)

	start, end := "A", "G"
	fmt.Printf("~~~ start=%s, end=%s\n", start, end)

	//
	queue := [][]*Node{[]*Node{a}}

LOOP1:
	for i := 0; i < len(queue); i++ {
		p := queue[i]
		for _, v := range p[len(p)-1].Next {
			pc := make([]*Node, len(p)+1)
			copy(pc, p)
			pc[len(pc)-1] = v
			if v.Value == end {
				fmt.Printf("--> Found %v\n", pc)
				break LOOP1
			} else {
				queue = append(queue, pc)
			}
		}
		// queue = queue[i+1:]
		// i--
	}

	fmt.Println("~~~", queue)

	//
	queue = [][]*Node{[]*Node{a}}

LOOP2:
	for len(queue) > 0 {
		p := queue[0]
		for _, v := range p[len(p)-1].Next {
			pc := make([]*Node, len(p)+1)
			copy(pc, p)
			pc[len(pc)-1] = v
			if v.Value == end {
				fmt.Printf("--> Found %v\n", pc)
				break LOOP2
			} else {
				queue = append(queue, pc)
			}
		}

		queue = queue[1:]
	}

	fmt.Println("~~~", queue)
}

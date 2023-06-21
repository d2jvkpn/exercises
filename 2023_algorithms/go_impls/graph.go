package go_impls

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

	fmt.Printf("--> v1: %v\n", a.v1(end))
	fmt.Printf("--> v2: %v\n", a.v2(end))
}

func (node *Node) v1(end string) []*Node {
	if node.Value == end {
		return []*Node{node}
	}

	queue := [][]*Node{[]*Node{node}}
	for i := 0; i < len(queue); i++ {
		p := queue[i]
		for _, v := range p[len(p)-1].Next {
			pc := make([]*Node, len(p)+1)
			copy(pc, p)
			pc[len(pc)-1] = v
			if v.Value == end {
				return pc
			} else {
				queue = append(queue, pc)
			}
		}
		// queue = queue[i+1:]
		// i--
	}

	return nil
}

func (node *Node) v2(end string) []*Node {
	if node.Value == end {
		return []*Node{node}
	}

	for queue := [][]*Node{[]*Node{node}}; ; {
		if len(queue) == 0 {
			return nil
		}

		p := queue[0]
		for _, v := range p[len(p)-1].Next {
			pc := make([]*Node, len(p)+1)
			copy(pc, p)
			pc[len(pc)-1] = v
			if v.Value == end {
				return pc
			} else {
				queue = append(queue, pc)
			}
		}

		// queue = append([][]*Node{}, queue[1:]...)
		copy(queue, queue[1:])       // drop the first element by left shift 1
		queue = queue[:len(queue)-1] // soft remove the last element
	}
}

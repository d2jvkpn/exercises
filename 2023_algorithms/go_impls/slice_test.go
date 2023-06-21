package go_impls

import (
	"fmt"
	"testing"
)

func TestSlice(t *testing.T) {
	s1 := make([]int, 3, 5)
	fmt.Printf("~~~ s1: %v, len=%d, cap=%d\n", s1, len(s1), cap(s1))

	s2 := s1[1:]
	fmt.Printf("~~~ s2: %v, len=%d, cap=%d\n", s2, len(s2), cap(s2))

	s1[1], s1[2] = 1, 2
	fmt.Printf("~~~ s1: %v, len=%d, cap=%d\n", s1, len(s1), cap(s1))

	copy(s1, s1[1:])
	fmt.Printf("~~~ s1: %v, len=%d, cap=%d\n", s1, len(s1), cap(s1))

	s1 = s1[:2]
	fmt.Printf("~~~ s1: %v, len=%d, cap=%d\n", s1, len(s1), cap(s1))

	s1 = append(s1, 3)
	fmt.Printf("~~~ s1: %v, len=%d, cap=%d\n", s1, len(s1), cap(s1))
}

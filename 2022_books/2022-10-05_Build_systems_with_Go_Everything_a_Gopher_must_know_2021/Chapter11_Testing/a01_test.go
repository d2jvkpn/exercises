package main

import (
	"fmt"
	"testing"
)

func TestMe(t *testing.T) {
	r := 2 + 2
	// t.Skip("Hello,", "testing")
	fmt.Println("=====", testing.Short())

	if r != 4 {
		t.Error("expected 4 got:", r)
	}

	t.Run("test1", test1)
	t.Run("test2", test2)
}

func test1(t *testing.T) {
	fmt.Println("this is test1")
}

func test2(t *testing.T) {
	fmt.Println("this is test2")
}

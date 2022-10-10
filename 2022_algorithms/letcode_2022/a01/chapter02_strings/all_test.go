package main

import (
	"fmt"
	"testing"
)

func TestIsPalindrome(t *testing.T) {
	fmt.Println(IsPalindrome("OP"))
}

func TestMyAtoi(t *testing.T) {
	fmt.Println(MyAtoi("a42"))
	fmt.Println(MyAtoi("a-42"))
	fmt.Println(MyAtoi("words and 987"))
}

func TestStrStr(t *testing.T) {
	fmt.Println(StrStr("hello", "ll"))
}

package main

import (
	"fmt"
	"testing"
)

func TestIsPalindrome(t *testing.T) {
	fmt.Println(IsPalindrome("0P"))
}

func TestMyAtoi(t *testing.T) {
	fmt.Println(MyAtoi("a42"))
	fmt.Println(MyAtoi("a-42"))
	fmt.Println(MyAtoi("words and 987"))

	fmt.Println(MyAtoi("-91283472332"))
}

func TestStrStr(t *testing.T) {
	fmt.Println(StrStr("hello", "ll"))
}

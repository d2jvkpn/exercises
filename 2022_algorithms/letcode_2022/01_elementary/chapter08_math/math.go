package main

import (
// "fmt"
)

func isPowerOfThree(n int) bool {
	for n > 0 {
		if n == 1 {
			return true
		}
		if n%3 != 0 {
			return false
		}
		n /= 3
	}
	return false
}

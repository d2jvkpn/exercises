package main

import (
	"fmt"
)

func ReverseString(s []byte) {
	for i := 0; i < len(s)/2; i++ {
		s[i], s[len(s)-i-1] = s[len(s)-i-1], s[i]
	}
}

func ReverseInteger(x int) int {
	isNeg := x < 0
	if isNeg {
		x = -x
	}

	arr := make([]int, 0)
	for ; x > 0; x /= 10 {
		arr = append(arr, x%10)
	}

	x = 0
	for i := range arr {
		for z := 0; z < len(arr)-i-1; z++ {
			arr[i] *= 10
		}
		x += arr[i]
	}

	if isNeg {
		x = -x
	}

	return x
}

func IsAnagram(s string, t string) bool {
	if len(s) != len(t) || len(s) == 0 {
		return false
	}

	rs, rt := []rune(s), []rune(t)
	ms, mt := make(map[rune]int, len(s)), make(map[rune]int, len(t))
	for i := range rs {
		ms[rs[i]]++
		mt[rt[i]]++
	}

	for k := range ms {
		if ms[k] != mt[k] {
			return false
		}
	}

	return true
}

func IsPalindrome(s string) bool {
	bts := make([]byte, 0, len(s))
	for _, b := range []byte(s) {
		if b < 'A' || b > 'z' {
			continue
		}
		if b < 'a' {
			b += 'a' - 'A'
		}
		bts = append(bts, b)
	}

	for i := 0; i < len(bts)/2; i++ {
		if bts[i] != bts[len(bts)-i-1] {
			return false
		}
	}

	return true
}

func MyAtoi(s string) int {
	bts := []byte(s)
	nn := make([]byte, 0, len(bts))
	first, last := -1, -1

	for i, b := range bts {
		ok := b >= '0' && b <= '9'
		if last > -1 && !ok {
			break
		}
		if !ok {
			continue
		}
		if last == -1 {
			first = i
		}
		nn = append(nn, b)
		last = i
	}

	out := 0
	if len(nn) == 0 {
		return out
	}

	for i := range nn {
		v := int(nn[i] - '0')
		fmt.Println("~~~", v)
		for j := 1; j < len(nn)-i; j++ {
			v *= 10
		}
		out += v
	}

	if first > 0 && bts[first-1] == '-' {
		out = -out
	}

	return out
}

func StrStr(haystack string, needle string) int {
	if len(haystack) < len(needle) {
		return -1
	}

	n := len(needle)
	for i := 0; i < len(haystack)-n+1; i++ {
		fmt.Println("~~~", i)
		if needle == haystack[i:i+n] {
			return i
		}
	}
	return -1
}

func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}

	bts := make([]byte, 0)

LOOP:
	for i := 0; i < len(strs[0]); i++ {
		b := strs[0][i]

		for _, v := range strs[1:] {
			if i > len(v)-1 || v[i] != b {
				break LOOP
			}
		}

		bts = append(bts, b)
	}

	return string(bts)
}

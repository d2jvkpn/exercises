// append.go
package main

import (
	"fmt"
)

func main() {
	a := []int{1, 2, 3}
	b := append(a[:1], 10)
	fmt.Printf(">>> a=%v, b=%v\n", a, b)
	fmt.Printf("    &a=%p, &b=%p\n", a, b)

	a[0] = 100
	fmt.Printf(">>> a=%v, b=%v\n", a, b)
	fmt.Printf("    &a=%p, &b=%p\n", a, b)

	b = append(b, 4)
	a[0] = 99
	fmt.Printf(">>> a=%v, b=%v\n", a, b)
	fmt.Printf("    &a=%p, &b=%p\n", a, b)

	slice := make([]int, 0, 3)
	fmt.Printf(">>> slice=%v, &slice=%p\n", slice, slice)
	slice = append(slice, 1)
	fmt.Printf(">>> slice=%v, &slice=%p\n", slice, slice)
	fmt.Printf("    len(slice)=%d, cap(slice)=%d\n", len(slice), cap(slice))

	append2(slice)
	fmt.Printf(">>> slice=%v, &slice=%p\n", slice, slice)
}

func append2(slice []int) {
	fmt.Printf("--- 1 slice=%v, &slice=%p\n", slice, slice)
	slice = append(slice, 2)
	fmt.Printf("--- 2 slice=%v, &slice=%p\n", slice, slice)
	slice = append(slice, 3)
	fmt.Printf("--- 3 slice=%v, &slice=%p\n", slice, slice)
	slice[0] = 9
	slice = append(slice, 4)
	fmt.Printf("--- 4 slice=%v, &slice=%p\n", slice, slice)
	fmt.Printf("    len(slice)=%d, cap(slice)=%d\n", len(slice), cap(slice))
}

/*
output:
>>> a=[1 10 3], b=[1 10]
    &a=0xc000016210, &b=0xc000016210
>>> a=[100 10 3], b=[100 10]
    &a=0xc000016210, &b=0xc000016210
>>> a=[99 10 4], b=[99 10 4]
    &a=0xc000016210, &b=0xc000016210
>>> slice=[], &slice=0xc000016228
>>> slice=[1], &slice=0xc000016228
    len(slice)=1, cap(slice)=3
--- 1 slice=[1], &slice=0xc000016228
--- 2 slice=[1 2], &slice=0xc000016228
--- 3 slice=[1 2 3], &slice=0xc000016228
--- 4 slice=[9 2 3 4], &slice=0xc0000182a0
    len(slice)=4, cap(slice)=6
>>> slice=[9], &slice=0xc000016228
*/

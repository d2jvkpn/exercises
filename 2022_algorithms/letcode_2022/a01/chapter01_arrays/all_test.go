package main

import (
	"fmt"
	"testing"
)

func TestRotateX1(t *testing.T) {
	arr := []int{1, 2, 3, 4, 5, 6, 7}
	RotateX1(arr, 3)
}

func TestRotate(t *testing.T) {
	arr := []int{1, 2, 3, 4, 5, 6, 7}
	Rotate(arr, 3)
	fmt.Println(arr)
}

func TestSingleNumber(t *testing.T) {
	arr := []int{2, 2, 1}
	fmt.Println(SingleNumber(arr))
}

func TestPlusOne(t *testing.T) {
	out := PlusOne([]int{9})
	fmt.Println(out)
}

func TestMoveZeroes(t *testing.T) {
	arr := []int{0, 1, 0, 3, 12}
	MoveZeroes(arr)
	fmt.Println(arr)
}

func TestIsValidSudoku(t *testing.T) {
	mat := [][]byte{
		{'.', '.', '.', '.', '5', '.', '.', '1', '.'},
		{'.', '4', '.', '3', '.', '.', '.', '.', '.'},
		{'.', '.', '.', '.', '.', '3', '.', '.', '1'},
		{'8', '.', '.', '.', '.', '.', '.', '2', '.'},
		{'.', '.', '2', '.', '7', '.', '.', '.', '.'},
		{'.', '1', '5', '.', '.', '.', '.', '.', '.'},
		{'.', '.', '.', '.', '.', '2', '.', '.', '.'},
		{'.', '2', '.', '9', '.', '.', '.', '.', '.'},
		{'.', '.', '4', '.', '.', '.', '.', '.', '.'},
	}

	fmt.Println(IsValidSudoku(mat))
}

func TestMaxProfit(t *testing.T) {
	arr := []int{7, 1, 5, 3, 6, 4}
	// arr := []int{1, 2, 3, 4, 5}
	fmt.Println("~~~", arr)
	fmt.Println(MaxProfit(arr))
}

package main

import (
	"fmt"
)

func MaxProfitBrute(prices []int) int {
	getMax := func(arr []int) int {
		// len(arr) > 1
		max := arr[0]
		for _, v := range arr[1:] {
			if v > max {
				max = v
			}
		}
		return max
	}

	max := 0
	for i := range prices[:len(prices)-1] {
		m := getMax(prices[i+1:]) - prices[i]
		if m > max {
			max = m
		}
	}

	return max
}

func MaxProfit(prices []int) int {
	min, max := prices[0], prices[0]
	p := 0

	for i := 1; i < len(prices); i++ {
		if v := prices[i] - min; v > 0 && v > p {
			p, max = v, prices[i]
			continue
		}
		
		if v2 := max - prices[i]; v2 > p {
			p, min = v2, prices[i]
		}
	}

	return p
}

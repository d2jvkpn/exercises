package main

import (
	// "fmt"
)

type Blockchain struct {
	blocks []*Block
}

func (bc *Blockchain) AddBlock(data string) *Blockchain {
	prevBlock := bc.blocks[len(bc.blocks)-1]
	newBlock := NewBlock(data, prevBlock.Hash)
	bc.blocks = append(bc.blocks, newBlock)

	return bc
}

func NewBlockchain() *Blockchain {
	return &Blockchain{[]*Block{NewGenesisBlock()}}
}

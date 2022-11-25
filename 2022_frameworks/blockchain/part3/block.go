package main

import (
	"bytes"
	"encoding/gob"
	"fmt"
	"time"
)

type Block struct {
	Timestamp     int64
	Data          []byte
	PrevBlockHash []byte
	Hash          []byte
	Nonce         int
}

func (b Block) String() string {
	return fmt.Sprintf(
		"Timestamp: %s, Data: %s, PrevBlockHash: %x, Hash: %x, Nonce: %d",
		time.Unix(b.Timestamp, 0).Format(time.RFC3339), b.Data,
		b.PrevBlockHash, b.Hash, b.Nonce,
	)
}

func NewBlock(data string, prevBlockHash []byte) *Block {
	block := &Block{time.Now().Unix(), []byte(data), prevBlockHash, []byte{}, 0}
	pow := NewProofOfWork(block)
	nonce, hash := pow.Run()

	block.Hash = hash[:]
	block.Nonce = nonce

	return block
}

func NewGenesisBlock() *Block {
	return NewBlock("Genesis Block", []byte{})
}

// Serialize serializes the block
func (b *Block) Serialize() (bts []byte, err error) {
	buf := new(bytes.Buffer)
	encoder := gob.NewEncoder(buf)

	if err = encoder.Encode(b); err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}

func DeserializeBlock(d []byte) (block *Block, err error) {
	decoder := gob.NewDecoder(bytes.NewReader(d))

	block = new(Block)
	if err = decoder.Decode(block); err != nil {
		return nil, err
	}

	return block, nil
}

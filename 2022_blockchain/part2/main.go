// https://jeiwan.net/posts/building-blockchain-in-go-part-2/
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"fmt"
	"math"
	"math/big"
	"time"
)

const (
	_TargetBits = 20 // 24
)

func main() {
	bc := NewBlockchain().AddBlock("Send 1 BTC to Ivan").AddBlock("Send 2 more BTC to Ivan")

	for i := range bc.blocks {
		fmt.Println(bc.blocks[i])
	}

	for idx, block := range bc.blocks {
		pow := NewProofOfWork(block)
		fmt.Printf("Block: %d, PoW: %t\n", idx, pow.Validate())
	}
}

type Block struct {
	Timestamp     int64
	Data          []byte
	PrevBlockHash []byte
	Hash          []byte
	Nonce         int
}

func (b Block) String() string {
	return fmt.Sprintf(
		"Timestamp: %s\n    Data: %s\n    PrevBlockHash: %x\n    Hash: %x\n    Nonce: %d\n",
		time.Unix(b.Timestamp, 0), b.Data,
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

type Blockchain struct {
	blocks []*Block
}

func (bc *Blockchain) AddBlock(data string) *Blockchain {
	prevBlock := bc.blocks[len(bc.blocks)-1]
	newBlock := NewBlock(data, prevBlock.Hash)
	bc.blocks = append(bc.blocks, newBlock)

	return bc
}

func NewGenesisBlock() *Block {
	return NewBlock("Genesis Block", []byte{})
}

func NewBlockchain() *Blockchain {
	return &Blockchain{[]*Block{NewGenesisBlock()}}
}

type ProofOfWork struct {
	block  *Block
	target *big.Int
}

func NewProofOfWork(b *Block) *ProofOfWork {
	target := big.NewInt(1)
	target.Lsh(target, uint(256-_TargetBits))

	return &ProofOfWork{b, target}
}

func (pow *ProofOfWork) prepareData(nonce int) []byte {
	data := bytes.Join(
		[][]byte{
			IntToHex(pow.block.Timestamp), pow.block.Data, pow.block.PrevBlockHash,
			IntToHex(_TargetBits), IntToHex(nonce),
		},
		[]byte{},
	)

	return data
}

func (pow *ProofOfWork) Run() (int, []byte) {
	var (
		hashInt big.Int
		hash    [32]byte
		data    []byte
	)
	start, nonce := time.Now(), 0

	fmt.Printf(
		">>> %s, Mining the block containing %q\n",
		start.Format(time.RFC3339), pow.block.Data,
	)

	for nonce < math.MaxInt64 {
		data = pow.prepareData(nonce)
		hash = sha256.Sum256(data)
		fmt.Printf("\r%x", hash)
		hashInt.SetBytes(hash[:])

		// fmt.Printf("\n~~~ %s\n    %s\n", hashInt.String(), pow.target.String())
		if hashInt.Cmp(pow.target) == -1 { // less than
			break
		} else {
			nonce++
		}
	}

	// fmt.Println("\n~~~", hashInt, pow.target)
	fmt.Printf("\n    %d times, %s\n\n", nonce+1, time.Since(start))

	return nonce, hash[:]
}

func (pow *ProofOfWork) Validate() bool {
	var hashInt big.Int

	data := pow.prepareData(pow.block.Nonce)
	hash := sha256.Sum256(data)
	hashInt.SetBytes(hash[:])

	return hashInt.Cmp(pow.target) == -1
}

func IntToHex[T int64 | int](num T) []byte {
	buff := new(bytes.Buffer)
	_ = binary.Write(buff, binary.BigEndian, int64(num)) // ignore error

	return buff.Bytes()
}

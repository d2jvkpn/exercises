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

func IntToHex(num int64) []byte {
	buff := new(bytes.Buffer)
	_ = binary.Write(buff, binary.BigEndian, num) // ignore error

	return buff.Bytes()
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

func (pow *ProofOfWork) prepareData(nonce int64) []byte {
	data := bytes.Join(
		[][]byte{
			IntToHex(pow.block.Timestamp),
			pow.block.Data,
			pow.block.PrevBlockHash[:],
			IntToHex(_TargetBits),
			IntToHex(nonce),
		},
		[]byte{},
	)

	return data
}

func (pow *ProofOfWork) Run() (int64, [32]byte) {
	var (
		hashInt big.Int
		hash    [32]byte
		data    []byte
	)
	start, nonce := time.Now(), int64(0)

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

	return nonce, hash
}

func (pow *ProofOfWork) Validate() bool {
	var hashInt big.Int

	data := pow.prepareData(pow.block.Nonce)
	hash := sha256.Sum256(data)
	hashInt.SetBytes(hash[:])

	return hashInt.Cmp(pow.target) == -1
}

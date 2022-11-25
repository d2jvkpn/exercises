package main

import (
	"fmt"
	"log"

	"github.com/boltdb/bolt"
)

// Blockchain keeps a sequence of Blocks
type Blockchain struct {
	bucket   []byte // blockchain buckey key in blotdb
	lastHash []byte // hash of last block
	dbPath   string
	db       *bolt.DB
}

// BlockchainIterator is used to iterate over blockchain blocks
type BlockchainIterator struct {
	currentHash []byte
	bc          *Blockchain
	db          *bolt.DB
}

func (bc *Blockchain) lastHashKey() []byte {
	return []byte("lastHashKey")
}

// NewBlockchain creates a new Blockchain with genesis Block
func NewBlockchain(dbPath, bucket string) (bc *Blockchain, err error) {
	bc = &Blockchain{dbPath: dbPath, bucket: []byte(bucket)}

	if bc.db, err = bolt.Open(bc.dbPath, 0600, nil); err != nil {
		return nil, err
	}

	err = bc.db.Update(func(tx *bolt.Tx) (err error) {
		var bucket *bolt.Bucket

		if bucket = tx.Bucket(bc.bucket); bucket != nil {
			bc.lastHash = bucket.Get(bc.lastHashKey())
			return nil
		}

		if bucket, err = tx.CreateBucket(bc.bucket); err != nil {
			return err
		}

		log.Println("No existing blockchain found. Creating a new one...")
		genesis := NewGenesisBlock()
		bts, _ := genesis.Serialize()
		bc.lastHash = genesis.Hash
		if err = bucket.Put(bc.lastHash, bts); err != nil {
			return err
		}
		bc.lastHash = genesis.Hash
		if err = bucket.Put(bc.lastHashKey(), genesis.Hash); err != nil {
			return err
		}

		return nil
	})

	if err != nil {
		return nil, err
	}
	return bc, nil
}

// Iterator ...
func (bc *Blockchain) Iterator() *BlockchainIterator {
	return &BlockchainIterator{currentHash: bc.lastHash, bc: bc, db: bc.db}
}

// AddBlock saves provided data as a block in the blockchain
func (bc *Blockchain) AddBlock(data string) (err error) {
	var (
		lastHash []byte
		block    *Block
	)

	err = bc.db.View(func(tx *bolt.Tx) error {
		bucket := tx.Bucket(bc.bucket)
		lastHash = bucket.Get(bc.lastHashKey())
		return nil
	})

	if err != nil {
		return err
	}

	block = NewBlock(data, lastHash)

	err = bc.db.Update(func(tx *bolt.Tx) (err error) {
		bucket := tx.Bucket(bc.bucket)
		bts, _ := block.Serialize()

		if err = bucket.Put(block.Hash, bts); err != nil {
			return err
		}

		if err = bucket.Put(bc.lastHashKey(), block.Hash); err != nil {
			return err
		}

		bc.lastHash = block.Hash
		return nil
	})

	if err != nil {
		log.Printf("!!! AddBlock: %v\n", err)
		return err
	}

	return nil
}

// Next returns next block starting from the lastHash
func (iter *BlockchainIterator) Next() *Block {
	var (
		err   error
		block *Block
	)

	err = iter.db.View(func(tx *bolt.Tx) error {
		bucket := tx.Bucket(iter.bc.bucket)
		bts := bucket.Get(iter.currentHash)

		if len(bts) == 0 {
			return fmt.Errorf("not exist")
		}
		block, _ = DeserializeBlock(bts)
		return nil
	})

	if err != nil {
		// log.Printf("!!! Next: %v\n", err)
		return nil
	}

	iter.currentHash = block.PrevBlockHash
	return block
}

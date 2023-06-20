package go_impls

import (
	"fmt"
	"testing"
)

func TestTrie(t *testing.T) {
	var (
		ok   bool
		trie *Trie
	)

	trie = NewTrie()

	//
	if ok = trie.Search("apple"); ok {
		t.Fatal("unexpected:", ok)
	}
	trie.Insert("apple")
	fmt.Printf("~~~ trie: %v\n", trie)
	if ok = trie.Search("apple"); !ok {
		t.Fatal("unexpected:", ok)
	}

	//
	if ok = trie.Search("ape"); ok {
		t.Fatal("unexpected:", ok)
	}
	trie.Insert("ape")
	fmt.Printf("~~~ trie: %v\n", trie)
	if ok = trie.Search("ape"); !ok {
		t.Fatal("unexpected:", ok)
	}
}

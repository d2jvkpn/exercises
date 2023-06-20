package go_impls

import (
// "fmt"
)

// Prefix Tree
type Trie struct {
	Children []TrieNode
}

type TrieNode struct {
	Value    rune
	Word     bool
	Children []TrieNode
}

func NewTrie() *Trie {
	return &Trie{Children: make([]TrieNode, 0)}
}

func NewTrieNode(runes []rune) (node *TrieNode) {
	length := len(runes)
	if length == 0 {
		return nil
	}

	node = &TrieNode{
		Value:    runes[0],
		Word:     false,
		Children: make([]TrieNode, 0, 1),
	}

	if length == 1 {
		node.Word = true
	} else {
		node.Children = append(node.Children, *NewTrieNode(runes[1:]))
	}

	return node
}

func (node *TrieNode) Insert(runes []rune) {
	length := len(runes)
	if length == 0 {
		return
	}

	for i := range node.Children {
		n := &node.Children[i]
		if n.Value != runes[0] {
			continue
		}

		if length == 1 {
			n.Word = true
			return
		}

		n.Insert(runes[1:])
		return
	}

	node.Children = append(node.Children, *NewTrieNode(runes))
	return
}

func (node *TrieNode) Search(runes []rune) bool {
	length := len(runes)
	if length == 0 {
		return false
	}

	// find a child matches runes[0]
	for i := range node.Children {
		n := &node.Children[i]
		if n.Value != runes[0] {
			continue
		}

		if length == 1 {
			return true
		}
		return n.Search(runes[1:])
	}

	return false
}

func (trie *Trie) Insert(word string) {
	var (
		node  *TrieNode
		runes []rune
	)

	runes = []rune(word)
	if len(runes) == 0 {
		return
	}

	for i := range trie.Children {
		node = &trie.Children[i]
		if node.Value == runes[0] {
			node.Insert(runes[1:])
			return
		}
	}

	// fmt.Println("~~~ append child:", string(runes))
	trie.Children = append(trie.Children, *NewTrieNode(runes))
	return
}

func (trie *Trie) Search(word string) (exists bool) {
	var (
		node  *TrieNode
		runes []rune
	)

	runes = []rune(word)
	if len(runes) == 0 {
		return false
	}

	for i := range trie.Children {
		node = &trie.Children[i]
		if node.Value == runes[0] {
			return node.Search(runes[1:])
		}
	}

	return false
}

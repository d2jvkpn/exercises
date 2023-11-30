#include <iostream>
#include "include/help.h"

using namespace std;

class Node {
public:
	char  ch;
	bool  isTerminal;
	Node* next;

	Node(char ch) {
		this->ch = ch;
		this->isTerminal = false;
		this->next = NULL;
	}

	Node(char ch, Node* next) {
		this->ch = ch;
		this->isTerminal = false;
		this->next = next;
	}

	~Node() {
		if (next != NULL) {
			delete next;
		}

		cout << "!!! delete Node: " << ch << endl;
	}

	Node* find(char ch) {
		Node* temp = this;

		while (temp != NULL) {
			if (temp->ch == ch) {
				return temp;
			}

			temp = temp->next;
		}

		return NULL;
	}
};

class Trie {
	Node* root;

public:
	Trie() {
		root = new Node('\0');
	}

	~Trie() {
		if (root != NULL) {
			delete root;
		}

		cout << "!!! delete Trie: " << endl;
	}

	void insert(string word) {
		if (trim(word) == 0) {
			return;
		}

		Node* temp = root;
		Node* node = NULL;

		for (char c: word) {
			node = temp->find(c);

			if (node == NULL) {
				node = new Node(c, temp->next);
				temp->next = node;
			}

			temp = node;
		}

		temp->isTerminal = true;
	}

	bool search(string word) {
		if (trim(word) == 0) {
			return false;
		}

		Node* temp = root;
		Node* node = NULL;

		for (char c: word) {
			node = temp->find(c);

			if (node == NULL) {
				return false;
			}

			temp = node;
		}

		return temp->isTerminal;
	}
};

int main() {
	// cout << "Hello, word!" << endl;
	string words[] = {"hello", "he", "apple", "ape", "news"};
	Trie trie;

	for (auto word: words) {
		cout << "==> Insert: " << word << endl;
		trie.insert(word);
	}

	string key;
	cout << "==> Search: ";
	getline(cin, key);

	bool ans = trie.search(key);

	if (ans) {
		cout << "Found: yes" << endl;
	} else {
		cout << "Found: no" << endl;
	}

	return 0;
}

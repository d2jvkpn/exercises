# include <iostream>
# include <unordered_map>
# include "lib/help.h"

using namespace std;

class Node {
public:
	char ch;
	bool isTerminal;
	unordered_map<char, Node*> umap;

	Node(char ch) {
		this->ch = ch;
		isTerminal = false;
	}

	~Node() {
		/*
		for (const auto &pair : umap) {
			cout << "Key: " << pair.first << " Value: " << pair.second << endl;
		}
		*/

		for (auto item = umap.begin(); item != umap.end(); item++) {
			// cout << "Key: " << item->first << " Value: " << item->second << endl;
			delete item->second;
		}
		// umap.erase(key);

		cout << "!!! delete Node: " << ch << endl;
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
		cout << "!!! delete Trie" << endl;
	}

	void insert(string word) {
		if (trim(word) == 0) {
			return;
		}

		Node* temp = root;

		for (char c: word) {
			if (temp->umap.count(c) == 0) { // 0: not found, 1: found
				temp->umap[c] = new Node(c);
			}

			temp = temp->umap[c];
		}

		temp->isTerminal = true;
	}

	bool search(string word) {
		if (trim(word) == 0) {
			return false;
		}

		Node* temp = root;

		for (char c: word) {
			if (temp->umap.count(c) == 0) { // 0: not found, 1: found
				return false;
			}

			temp = temp->umap[c];
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
	// cin >> key;
	getline(cin, key);

	if (trie.search(key)) {
		cout << "Found: yes" << endl;
	} else {
		cout << "Found: no" << endl;
	}

	return 0;
}

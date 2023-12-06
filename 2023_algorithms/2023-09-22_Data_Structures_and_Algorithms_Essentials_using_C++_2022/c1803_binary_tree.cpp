#include <iostream>
#include <string>
#include "include/node.h"

using namespace std;

template <typename T>
class Tree {
public:
	Node<T>* root;

	Tree() {
		root = NULL;
	}

	~Tree() {
		if (root != NULL) {
			delete root;
			this->root = NULL;
		}

		cout << "!!! delete Tree" << endl;
	}

	void build() {
		// Node* root = preorderBuild("root");
		Node<T>* root = levelsBuild();
		this->root = root;
	}

	void show() {
		if (root == NULL) {
			cout << "Tree { root: NULL }" << endl;
		}

		string s1, s2;

		s1 = (root->left == NULL) ? "NULL" : s1 + root->left->data;
		s2 = (root->right == NULL) ? "NULL" : s2 + root->right->data;

		cout << "Tree { root: " << root->data;
		cout << ", left: " << s1 << ", right: " << s2 << " }" << endl;
	}
};

int main() {
	// cout << "Hello, world!\n";

	Tree<string>* tree = new Tree<string>;
	tree->build();
	tree->show();

	delete tree;

	return 0;
}

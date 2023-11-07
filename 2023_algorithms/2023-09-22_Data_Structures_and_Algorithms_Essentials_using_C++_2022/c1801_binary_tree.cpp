# include <iostream>
# include <string>
# include "c1801_node.h"

using namespace std;

// string = to_string(int)
// 
Node* buildNode(string msg) {
	char data;

	cout << "Enter " << msg << ": ";
	cin >> data;

	if (data < 'a' || data > 'z') {
		return NULL;
	}

	Node* node = new Node(data);

	string s = "'";
	s += data;
	s += "'";
	node->left = buildNode(s + ".left");
	node->right = buildNode(s + ".right");

	return node;
}

class Tree {
public:
	Node* root;

	Tree() {
		root = NULL;
	}

	~Tree() {
		if (root != NULL) {
			delete root;
			root = NULL;
		}

		cout << "!!! delete Tree" << endl;
	}

	void build() {
		Node* root = buildNode("root node(a-z)");
		this->root = root;
	}

	void show() {
		if (root == NULL) {
			cout << "Tree{ root: NULL }" << endl;
		}

		string s1, s2;

		s1 = (root->left == NULL) ? "NULL" : s1 + root->left->data;
		s2 = (root->right == NULL) ? "NULL" : s2 + root->right->data;

		cout << "Tree{ root: '" << root->data;
		cout << "', left: " << s1 << ", right: " << s2 << "}" << endl;
	}
};

int main() {
	// cout << "Hello, world!\n";

	Node* node = new Node('a');
	node->left = new Node('b');
	node->show();
	node->right = new Node('c');

	delete node;

	//
	Tree* tree = new Tree;
	tree->build();
	tree->show();

	delete tree;

	return 0;
}

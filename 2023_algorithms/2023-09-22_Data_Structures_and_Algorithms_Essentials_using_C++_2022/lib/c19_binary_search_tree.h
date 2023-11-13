# include <iostream>
#include <array>
# include "c18_node.h"

using namespace std;

class Tree {
	Node<int>* root;

private:
	void insert_into_node(Node<int>* node, Node<int>* target) {
		if (node->data <= target->data) {
			if (target->left == NULL) {
				target->left = node;
			} else {
				insert_into_node(node, target->left);
			}
			return;
		}

		if (target->right == NULL) {
			target->right = node;
		} else {
			insert_into_node(node, target->right);
		}
	}

	void printOrderRecu(Node<int>* node) {
		if (node == NULL) {
			return;
		}

		printOrderRecu(node->left);
		cout << node->data << ", ";
		printOrderRecu(node->right);
	}

	bool searchRecu(Node<int>* node, int val) {
		if (node == NULL) {
			return false;
		}

		cout << "~~~ searchRecu: " << node->data << endl;

		if (node->data == val) {
			return true;
		} else if (val < node->data) {
			return searchRecu(node->left, val);
		} else {
			return searchRecu(node->right, val);
		}
	}

	array<Node<int>*, 2> searchLeaf(Node<int>* node, int val) {
		if (node == NULL) {
			return { NULL, NULL };
		}
	
		if (node->data == val) {
			return { NULL, node };
		} else if (val < node->data) {
			if (node->left == NULL) {
				return { NULL, NULL };
			} else if (node->left->data == val) {
				return { node, node->left };
			}
			return searchLeaf(node->left, val);
		} else {
			if (node->right == NULL) {
				return { NULL, NULL };
			} else if (node->right->data == val) {
				return { node, node->right };
			}
			return searchLeaf(node->right, val);
		}
	}

	array<Node<int>*, 2> dropMin(Node<int>* node) {
		if (node == NULL) {
			return { NULL, NULL }; // no min
		}

		Node<int>* left = node->left;

		if (left == NULL) {
			return { NULL, node }; // self
		}

		if (left->left == NULL) {
			node->left = left->right;
			left->right = NULL;
			return { node, left }; // {parent, left}
		}

		return dropMin(node->left);
	}

	array<Node<int>*, 2> dropMax(Node<int>* node) {
		if (node == NULL) {
			return { NULL, NULL }; // no max
		}

		Node<int>* right = node->right;

		if (right == NULL) {
			return { NULL, node }; // self
		}

		if (right->right == NULL) {
			node->right = right->left;
			right->left = NULL;
			return { node, right }; // {parent, right}
		}

		return dropMax(node->right); // 
	}

public:
	Tree() {
		root = NULL;
	}

	~Tree() {
		if (root != NULL) {
			delete root;
		}

		cout << "!!! delete Tree" << endl;
	}

	void levelsOrder() {
		if (root == NULL) {
			cout << "Tree{ root: NULL }" << endl;
		} else {
			root->levelsOrder();
		}
	}

	void printOrder() {
		printOrderRecu(root);
		cout << "\n";
	}

	Tree* insert(int value) {
		Node<int>* node = new Node<int>(value);

		if (root == NULL) {
			root = node;
		} else {
			insert_into_node(node, root);
		}

		return this;
	}

	bool search(int val) {
		return searchRecu(root, val);
	}

	bool remove(int val) {
		array<Node<int>*, 2> arr = searchLeaf(root, val);

		Node<int>* parent = arr[0];
		Node<int>* target = arr[1];
		Node<int>* replace;

		if (target == NULL) {
			return false; // not found
		}

		if (parent == NULL) { // match root node, as root node doesn't have parent node
			delete root; // TODO: fix
			this->root = NULL;
			return true;
		}

		// parent != NULL && target != NULL
		bool onLeft = parent->left == target;

		if (target->isLeaf()) {
			replace = NULL;
		} else if (target->left == NULL || target->right == NULL) { // target only has one child
			replace = target->right == NULL ? target->left : target->right;
		} else { // target has two children
			array<Node<int>*, 2> pair = dropMin(target->right);
			replace = pair[1];

			replace->left = target->left;
			replace->right = target->right;
		}

		if (onLeft) {
			parent->left = replace;
		} else {
			parent->right = replace;
		}

		target->left = NULL;
		target->right = NULL;
		delete target;

		return true;
	}
};

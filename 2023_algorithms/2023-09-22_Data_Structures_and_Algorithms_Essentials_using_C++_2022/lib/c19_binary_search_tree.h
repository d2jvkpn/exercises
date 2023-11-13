# include <iostream>
#include <array>
# include "c18_node.h"

using namespace std;

template <typename T>
class Tree {
	Node<T>* root;

private:
	void insert_into_node(Node<T>* node, Node<T>* target) {
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

	void printOrderRecu(Node<T>* node) {
		if (node == NULL) {
			return;
		}

		printOrderRecu(node->left);
		cout << node->data << ", ";
		printOrderRecu(node->right);
	}

	Node<T>* searchRecu(Node<T>* node, int val) {
		if (node == NULL) {
			return NULL;
		}

		// cout << "~~~ searchRecu: " << node->data << endl;

		if (node->data == val) {
			return node;
		} else if (val < node->data) {
			return searchRecu(node->left, val);
		} else {
			return searchRecu(node->right, val);
		}
	}

	array<Node<T>*, 2> searchLeaf(Node<T>* node, int val) {
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

	array<Node<T>*, 2> dropMin(Node<T>* node) {
		if (node == NULL) {
			return { NULL, NULL }; // no min
		}

		Node<T>* left = node->left;

		if (left == NULL) {
			return { NULL, node }; // self
		}

		if (left->left == NULL) {
			node->left = left->right;
			left->right = NULL;
			return { node, left };
		}

		return dropMin(node->left);
	}

	array<Node<T>*, 2> dropMax(Node<T>* node) {
		if (node == NULL) {
			return { NULL, NULL }; // no max
		}

		Node<T>* right = node->right;

		if (right == NULL) {
			return { NULL, node }; // self
		}

		if (right->right == NULL) {
			node->right = right->left;
			right->left = NULL;
			return { node, right };
		}

		return dropMax(node->right);
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
		Node<T>* node = new Node<T>(value);

		if (root == NULL) {
			root = node;
		} else {
			insert_into_node(node, root);
		}

		return this;
	}

	Node<T>* search(int val) {
		return searchRecu(root, val);
	}

	Node<T>* remove(int val) {
		array<Node<T>*, 2> arr = searchLeaf(root, val);

		Node<T>* parent = arr[0];
		Node<T>* target = arr[1];
		Node<T>* successor;
		array<Node<T>*, 2> pair;

		if (target == NULL) {
			return NULL; // not found
		}

		if (parent == NULL) { // root node doesn't have parent node
			pair = dropMin(target->right);
			successor = pair[1];
			successor->left = target->left;
		} else if (target->isLeaf()) { // is a leaf node
			successor = NULL;
		} else if (target->left == NULL || target->right == NULL) { // target only has one child
			successor = target->right == NULL ? target->left : target->right;
		} else { // target has two children
			pair = dropMin(target->right);
			successor = pair[1];

			successor->left = target->left;
			successor->right = target->right;
		}

		if (parent == NULL) {
			this->root = successor;
		} else if (parent->left == target) {
			parent->left = successor;
		} else {
			parent->right = successor;
		}

		target->left = NULL;
		target->right = NULL;
		// delete target;

		return target;
	}
};

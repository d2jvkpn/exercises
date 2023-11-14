# include <iostream>
# include <array>
# include <vector>
# include "c18_node.h"

using namespace std;

template <typename T>
class Tree {
private:
	void insert_into(Node<T>* node, Node<T>* target) {
		if (node->data <= target->data) {
			if (target->left == NULL) {
				target->left = node;
			} else {
				insert_into(node, target->left);
			}
			return;
		}

		if (target->right == NULL) {
			target->right = node;
		} else {
			insert_into(node, target->right);
		}
	}

	void printOrderRecur(Node<T>* node) {
		if (node == NULL) {
			return;
		}

		printOrderRecur(node->left);
		cout << node->data << ", ";
		printOrderRecur(node->right);
	}

	Node<T>* searchRecur(Node<T>* node, T val) {
		if (node == NULL) {
			return NULL;
		}

		// cout << "~~~ searchRecur: " << node->data << endl;

		if (node->data == val) {
			return node;
		} else if (val < node->data) {
			return searchRecur(node->left, val);
		} else {
			return searchRecur(node->right, val);
		}
	}

	array<Node<T>*, 2> searchLeaf(Node<T>* node, T val) {
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

	array<Node<T>*, 2> takeMin(Node<T>* node) {
		if (node == NULL) {
			return { NULL, NULL }; // not found
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

		return takeMin(node->left);
	}

	array<Node<T>*, 2> takeMax(Node<T>* node) {
		if (node == NULL) {
			return { NULL, NULL }; // not found
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

		return takeMax(node->right);
	}

	bool inRange(Node<T>* node, T low, T high) {
		return node != NULL && node->data >= low && node->data <= high;
	}

	void printRangeRecur(Node<T>* node, T low, T high) {
		if (node == NULL) {
			return;
		}

		printRangeRecur(node->left, low, high);

		if (node->data >= low && node->data <= high) {
			cout << node->data << ", ";
		}

		printRangeRecur(node->right, low, high);
	}

public:
	Node<T>* root;

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
		printOrderRecur(root);
		cout << "\n";
	}

	Tree* insert(T value) {
		Node<T>* node = new Node<T>(value);

		if (root == NULL) {
			root = node;
		} else {
			insert_into(node, root);
		}

		return this;
	}

	void insert_vec(vector<T> vec) {
		for (int i=0; i<vec.size(); i++) {
			this->insert(vec[i]);
		}
	}

	Node<T>* search(T val) {
		return searchRecur(root, val);
	}

	Node<T>* remove(T val) {
		array<Node<T>*, 2> arr = searchLeaf(root, val);

		Node<T>* parent = arr[0];
		Node<T>* target = arr[1];
		array<Node<T>*, 2> pair;
		Node<T>* successor;

		if (target == NULL) {
			return NULL; // not found
		}

		if (parent == NULL) { // root node
			pair = takeMin(target->right);
			successor = pair[1];

			if (successor == NULL) { // target->right == NULL
				successor = target->left;
			} else {
				successor->left = target->left;
			}
		} else if (target->isLeaf()) { // a leaf node
			successor = NULL;
		} else if (target->left == NULL || target->right == NULL) { // target only has one child
			successor = target->right == NULL ? target->left : target->right;
		} else { // target has two children
			pair = takeMin(target->right);
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

	void printRange(T low, T high) {
		printRangeRecur(root, low, high);
		cout << endl;
	}
};

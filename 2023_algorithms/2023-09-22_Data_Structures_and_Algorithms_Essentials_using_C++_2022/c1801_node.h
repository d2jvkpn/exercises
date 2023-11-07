# include <iostream>
# include <string>

using namespace std;

class Node {
public:
	char  data;
	Node* left;
	Node* right;

	Node(char data) {
		this->data = data;
		left = NULL;
		right = NULL;
	}

	~Node() {
		this->clear();

		cout << "!!! delete Node: " << data << endl;
	}

	void show() {
		string s1, s2;

		s1 = (left == NULL) ? "NULL" : s1 + left->data;
		s2 = (right == NULL) ? "NULL" : s2 + right->data;

		cout << "Node{ data: '" << data << "', left: " << s1 << ", right: " << s2 << " }" << endl;
	}

	bool isLeaf() {
		return left == NULL && right == NULL;
	}

private:
	void clear() {
		if (left != NULL) {
			if (left->isLeaf()) {
				left->clear();
			}
			delete left;
			left = NULL;
		}

		if (right != NULL) {
			if (right->isLeaf()) {
				right->clear();
			}
			delete right;
			right = NULL;
		}

		// can't delete self/this
	}
};

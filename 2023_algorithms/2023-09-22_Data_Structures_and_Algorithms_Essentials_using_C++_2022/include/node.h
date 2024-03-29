#include <iostream>
#include <string>
#include <queue>
#include <array>
#include <sstream>

using namespace std;

template <typename T>
using Queue = queue<T>;

template <typename F, typename S>
using Pair = pair<F, S>;

template <typename T, size_t N>
using Array = array<T, N>;

template <typename T>
class Node {
private:
	void clear() {
		if (this->left != NULL) {
			this->left->clear();
			delete this->left;
			this->left = NULL;
		}

		if (this->right != NULL) {
			this->right->clear();
			delete this->right;
			this->right = NULL;
		}

		// can't delete self/this
	}

public:
	T      data;
	Node*  left;
	Node*  right;

	Node(T data) {
		this->data = data;
		left = NULL;
		right = NULL;
	}

	~Node() {
		// this->clear();
		if (left != NULL) {
			delete left;
			left = NULL;
		}

		if (right != NULL) {
			delete right;
			right = NULL;
		}

		cout << "!!! delete Node: " << this->data << endl;
	}

	void show() {
		string s1, s2;

		s1 = (this->left == NULL) ? "NULL" : s1 + this->left->data;
		s2 = (this->right == NULL) ? "NULL" : s2 + this->right->data;

		cout << "Node { data: " << data << ", left: " << s1 << ", right: " << s2 << " }" << endl;
	}

	string toString() {
		if (this == NULL) {
			return "NULL";
		}

		string s1 = (this->left == NULL) ? "" : this->left->data;
		string s2 = (this->right == NULL) ? "" : this->right->data;

		stringstream ss;

		ss << data << "(" << s1 << ", " << s2 << ")";

		return ss.str();
	}

	bool isLeaf() {
		return this->left == NULL && this->right == NULL;
	}

	int count() {
		if (this == NULL) {
			return 0;
		}

		int ans = 1;

		if (this->left != NULL) {
			ans += this->left->count();
		}

		if (this->right != NULL) {
			ans += this->right->count();
		}

		return ans;
	}

	int height() {
		// Pair<int, int> pair = height2();
		// return pair.first > pair.second ? pair.first + 1: pair.second + 1;

		Array<int, 2> pair = height2();
		return pair[0] > pair[1] ? pair[0] + 1: pair[1] + 1;
	}

	Array<int, 2> height2() {
		Array<int, 2> ans = {0, 0};

		if (this->left != NULL) {
			ans[0] = this->left->height();
		}

		if (this->right != NULL) {
			ans[1] = this->right->height();
		}

		return ans;
	}

	int diameter() {
		int h1 = this->left == NULL ? 0 : this->left->height();
		int h2 = this->right == NULL ? 0 : this->right->height();

		int h = h1 + h2;
		int d1 = this->left == NULL ? 0 : this->left->diameter();
		int d2 = this->right == NULL ? 0 : this->right->diameter();

		return max(h, max(d1, d2));
	}

	// preorder traversal
	void preorder() {
		cout << "==> Preorder Order Traversal:" << endl;
		if (this == NULL) {
			return;
		}

		// cout << this->data << "->";
		cout << this->toString() << "->";

		if (this->left != NULL) {
			this->left->preorder();
		}

		if (this->right != NULL) {
			this->right->preorder();
		}
	}

	// inorder traversal
	void inorder() {
		cout << "==> Inorder Order Traversal:" << endl;
		if (this == NULL) {
			return;
		}

		if (this->left != NULL) {
			this->left->inorder();
		}

		// cout << this->data << "->";
		cout << this->toString() << "->";

		if (this->right != NULL) {
			this->right->inorder();
		}
	}

	// post traversal
	void postorder() {
		cout << "==> Post Order Traversal:" << endl;
		if (this == NULL) {
			return;
		}

		if (this->left != NULL) {
			this->left->postorder();
		}

		if (this->right != NULL) {
			this->right->postorder();
		}

		// cout << this->data << "->";
		cout << this->toString() << "->";
	}

	void levelsOrder() {
		cout << "==> Levels Order Traversal:" << endl;
		if (this == NULL) {
			return;
		}

		Queue<Node*> queue;
		Node* temp;

		queue.push(this);

		while(!queue.empty()) {
			temp = queue.front();
			queue.pop();

			if (temp == NULL) {
				break;
			}

			cout << temp->toString() << "->";

			if (temp->left != NULL) {
				queue.push(temp->left);
			}

			if (temp->right != NULL) {
				queue.push(temp->right);
			}
		}

		cout << endl;
	}

	// tree is balanced, if ans in {-1, 0, 1}
	int balanceFactor() {
		return left.count() - right.count();
	}

	void rotateLeft() {
		Node* subtree = this->right;
		if (subtree == NULL) {
			return;
		}

		this->right = subtree->left;
		subtree->left = NULL;

		/* WRONG
		swap(*this, *subtree);
		this->left = subtree;
		*/

		swap(this->data, subtree->data);
		swap(this->left, subtree->left);
		swap(this->right, subtree->right);
		this->left = subtree;
	}

	void rotateRight() {
		Node* subtree = this->left;
		if (subtree == NULL) {
			return;
		}

		this->left = subtree->right;
		subtree->right = NULL;

		swap(this->data, subtree->data);
		swap(this->left, subtree->left);
		swap(this->right, subtree->right);

		this->right = subtree;
	}
};

// string = to_string(int)
Node<string>* preorderBuild(string msg) {
	string data;

	cout << "Enter " << msg << ": ";
	// cin >> data;
	getline(cin, data);

	if (data.empty()) {
		return NULL;
	}

	Node<string>* node = new Node(data);

	string s = "";
	s += data;

	node->left = preorderBuild(s + ".left");
	node->right = preorderBuild(s + ".right");

	return node;
}

// string = to_string(int)
Node<string>* levelsBuild() {
	string               data;
	queue<Node<string>*> q;
	Node<string>*        node;
	Node<string>*        temp;

	cout << "Enter node (\"\" -> NULL, \".\" -> QUIT): ";
	// cin >> data;
	getline(cin, data);

	if (data.empty()) {
		return NULL;
	}

	node = new Node(data);

	q.push(node);

	while(!q.empty()) {
		temp = q.front();
		q.pop();

		if (temp == NULL) {
			break;
		}

		cout << "Enter " << temp->data << ".left: ";
		getline(cin, data);

		cout << "==> " << data << endl;
		if (data == ".") {
			break;
		} else if (!data.empty()) {
			Node<string>* left = new Node(data);
			temp->left = left;
			q.push(left);
		}

		cout << "Enter " << temp->data << ".right: ";
		getline(cin, data);

		if (data == ".") {
			break;
		} else if (!data.empty()) {
			Node<string>* right = new Node(data);
			temp->right = right;
			q.push(right);
		}
	}

	return node;
}

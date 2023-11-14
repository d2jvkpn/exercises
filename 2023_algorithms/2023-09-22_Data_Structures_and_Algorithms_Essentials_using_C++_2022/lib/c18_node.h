# include <iostream>
# include <string>
# include <queue>

using namespace std;

template <typename T>
class Node {
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
		this->clear();

		cout << "!!! delete Node: " << this->data << endl;
	}

	void show() {
		string s1, s2;

		s1 = (this->left == NULL) ? "NULL" : s1 + this->left->data;
		s2 = (this->right == NULL) ? "NULL" : s2 + this->right->data;

		cout << "Node { data: " << data << ", left: " << s1 << ", right: " << s2 << " }" << endl;
	}

	bool isLeaf() {
		return this->left == NULL && this->right == NULL;
	}

	int count() {
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
		int h1 = 0, h2 = 0;

		if (this->left != NULL) {
			h1 = this->left->height();
		}

		if (this->right != NULL) {
			h2 += this->right->height();
		}

		return h1 > h2 ? h1 + 1: h2 + 1;
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
		cout << data << "->";

		if (this->left != NULL) {
			this->left->preorder();
		}

		if (this->right != NULL) {
			this->right->preorder();
		}
	}

	// inorder traversal
	void inorder() {
		if (this->left != NULL) {
			this->left->inorder();
		}

		cout << this->data << "->";

		if (this->right != NULL) {
			this->right->inorder();
		}
	}

	// post traversal
	void postorder() {
		if (this->left != NULL) {
			this->left->postorder();
		}

		if (this->right != NULL) {
			this->right->postorder();
		}

		cout << this->data << "->";
	}

	void levelsOrder() {
		queue<Node*> q;
		Node* temp;

		q.push(this);

		while(!q.empty()) {
			temp = q.front();
			q.pop();

			if (temp == NULL) {
				break;
			}

			cout << temp->data << "-";

			if (temp->left != NULL) {
				q.push(temp->left);
			}

			if (temp->right != NULL) {
				q.push(temp->right);
			}
		}

		cout << endl;
	}

private:
	void clear() {
		if (this->left != NULL) {
			if (this->left->isLeaf()) {
				this->left->clear();
			}
			delete this->left;
			this->left = NULL;
		}

		if (this->right != NULL) {
			if (this->right->isLeaf()) {
				this->right->clear();
			}
			delete this->right;
			this->right = NULL;
		}

		// can't delete self/this
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
		}

		if (!data.empty()) {
			Node<string>* left = new Node(data);
			temp->left = left;
			q.push(left);
		}

		cout << "Enter " << temp->data << ".right: ";
		getline(cin, data);

		if (!data.empty()) {
			Node<string>* right = new Node(data);
			temp->right = right;
			q.push(right);
		}
	}

	return node;
}

# include <iostream>
# include <string>
# include <queue>

using namespace std;

class Node {
public:
	string  data;
	Node* left;
	Node* right;

	Node(string data) {
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

		cout << "Node { data: " << data << ", left: " << s1 << ", right: " << s2 << " }" << endl;
	}

	bool isLeaf() {
		return left == NULL && right == NULL;
	}

	int count() {
		int ans = 1;

		if (left != NULL) {
			ans += left->count();
		}

		if (right != NULL) {
			ans += right->count();
		}

		return ans;
	}

	int levels() {
		int left_levels = 0, right_levels = 0;

		if (left != NULL) {
			left_levels = left->levels();
		}

		if (right != NULL) {
			right_levels += right->levels();
		}

		return left_levels > right_levels ? left_levels + 1: right_levels + 1;
	}

	// preorder traversal
	void preorder() {
		cout << data << "->";

		if (left != NULL) {
			left->preorder();
		}

		if (right != NULL) {
			right->preorder();
		}
	}

	// inorder traversal
	void inorder() {
		if (left != NULL) {
			left->inorder();
		}

		cout << data << "->";

		if (right != NULL) {
			right->inorder();
		}
	}

	// post traversal
	void postorder() {
		if (left != NULL) {
			left->postorder();
		}

		if (right != NULL) {
			right->postorder();
		}

		cout << data << "->";
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

// string = to_string(int)
Node* preorderBuild(string msg) {
	string data;

	cout << "Enter " << msg << ": ";
	// cin >> data;
	getline(cin, data);

	if (data.empty()) {
		return NULL;
	}

	Node* node = new Node(data);

	string s = "";
	s += data;

	node->left = preorderBuild(s + ".left");
	node->right = preorderBuild(s + ".right");

	return node;
}

// string = to_string(int)
Node* levelsBuild() {
	string       data;
	queue<Node*> q;
	Node*        node;
	Node*        temp;

	cout << "Enter node: ";
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

		if (!data.empty()) {
			Node* left = new Node(data);
			temp->left = left;
			q.push(left);
		}

		cout << "Enter " << temp->data << ".right: ";
		getline(cin, data);

		if (!data.empty()) {
			Node* right = new Node(data);
			temp->right = right;
			q.push(right);
		}
	}

	return node;
}

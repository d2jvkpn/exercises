# include <iostream>
# include <string>

using namespace std;

class Node {
public:
	char  data;
	Node* left;
	Node* right;

	Node(int data) {
		this->data = data;
		left = NULL;
		right = NULL;
	}
};

// string = to_string(int)
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

int main() {
	// cout << "Hello, world!\n";

	Node* node = buildNode("root");

	return 0;
}

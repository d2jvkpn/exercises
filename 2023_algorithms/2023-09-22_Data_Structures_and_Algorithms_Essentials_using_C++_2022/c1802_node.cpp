# include <iostream>
# include <string>
# include "lib/c1801_node.h"

using namespace std;

void test01() {
	Node n1 = Node("a");

	Node* n2 = new Node("b");
	delete n2; // you must manual delete/free n2

	Node* n3 = &n1;
	// delete n3; // !!! double free or corruption (out)

	// auto delete/free n1 here
}

int main() {
	// cout << "Hello, world!\n";

	test01();

	Node* node = new Node("A");
	Node* b = new Node("B");
	Node* c = new Node("C");
	node->left = b;
	node->right = c;
	node->show();

	Node* d = new Node("D");
	Node* e = new Node("E");
	b->left = d;
	b->right = e;

	Node* f = new Node("F");
	Node* g = new Node("G");
	c->left = f;
	c->right = g;

	cout << "==> Preorder:" << endl;
	node->preorder();
	cout << endl;

	cout << "==> Inorder:" << endl;
	node->inorder();
	cout << endl;

	cout << "==> Postorder:" << endl;
	node->postorder();
	cout << endl;

	cout << "==> Delete Node:" << endl;
	delete node;

	return 0;
}

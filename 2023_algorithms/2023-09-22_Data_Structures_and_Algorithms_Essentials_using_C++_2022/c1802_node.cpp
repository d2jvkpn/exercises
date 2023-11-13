# include <iostream>
# include <string>
# include "lib/c18_node.h"

using namespace std;

void test01() {
	Node<string> n1 = Node<string>("a");

	Node<string>* n2 = new Node<string>("b");
	delete n2; // you must manual delete/free n2

	Node<string>* n3 = &n1;
	// delete n3; // !!! double free or corruption (out)

	// auto delete/free n1 here
}

int main() {
	// cout << "Hello, world!\n";

	test01();

	Node<string>* node = new Node<string>("A");
	Node<string>* b = new Node<string>("B");
	Node<string>* c = new Node<string>("C");
	node->left = b;
	node->right = c;
	node->show();

	Node<string>* d = new Node<string>("D");
	Node<string>* e = new Node<string>("E");
	b->left = d;
	b->right = e;

	Node<string>* f = new Node<string>("F");
	Node<string>* g = new Node<string>("G");
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

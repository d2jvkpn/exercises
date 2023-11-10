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

	// test01();

	Node* node = levelsBuild();
	if (node == NULL) {
		return 0;
	}

	cout << "==> Levels order:" << endl;
	node->levelsOrder();

	cout << "Node count: " << node->count() << ", height: " << node->height() << endl;

	cout << "Node diameter: " << node->diameter() << endl;

	cout << "==> Delete Node:" << endl;
	delete node;

	return 0;
}

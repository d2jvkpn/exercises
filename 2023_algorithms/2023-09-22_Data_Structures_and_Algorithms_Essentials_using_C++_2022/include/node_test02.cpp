#include <iostream>
#include "node.h"

using namespace std;

int main() {
	// cout << "Hello, world!\n";

	Node<string>* tree = levelsBuild();

	tree->levelsOrder();

	tree->rotateLeft();

	tree->levelsOrder();
	return 0;
}

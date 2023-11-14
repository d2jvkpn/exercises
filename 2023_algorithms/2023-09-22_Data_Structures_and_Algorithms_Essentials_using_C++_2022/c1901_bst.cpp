# include <iostream>
# include "lib/c19_binary_search_tree.h"

using namespace std;

int main() {
	// cout << "Hello, world!" << endl;
	Tree<int>* tree = new Tree<int>();

	tree->insert(8);
	/*->insert(3)->insert(10)->insert(1)->insert(6)
	  ->insert(14)->insert(4)->insert(7)->insert(13)->insert(19);*/

	tree->levelsOrder();
	tree->printOrder();

	cout << "==> Search 7: " << tree->search(7) << endl;
	cout << "==> Search 5: " << tree->search(5) << endl;

	int val;
	cout << "==> Enter node to delete: ";
	cin >> val;

	Node<int>* removed = tree->remove(val);
	cout << "==> Removed: " << removed << endl;

	tree->printOrder();

	cout << "==> DELETE Tree" << endl;
	delete tree;

	return 0;
}

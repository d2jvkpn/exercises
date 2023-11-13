# include <iostream>
# include "lib/c19_binary_search_tree.h"

using namespace std;

int main() {
	// cout << "Hello, world!" << endl;
	Tree* tree = new Tree();

	tree->insert(8)->insert(3)->insert(10)->insert(1)->insert(6)
	  ->insert(14)->insert(4)->insert(7)->insert(13);

	tree->levelsOrder();
	tree->printOrder();

	cout << "Search 7: \n" << tree->search(7) << endl;
	cout << "Search 5: \n" << tree->search(5) << endl;

	int val;
	cout << "Enter node to delete: ";
	cin >> val;

	tree->remove(val);
	tree->printOrder();

	cout << "==> delete Tree" << endl;
	delete tree;

	return 0;
}

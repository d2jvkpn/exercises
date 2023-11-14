# include <iostream>
# include <vector>
# include "lib/c19_binary_search_tree.h"

using namespace std;

void test01(vector<int> vec, int val) {
	// cout << "Hello, world!" << endl;
	Tree<int>* tree = new Tree<int>();

	/*
	tree->insert(8)->insert(3)->insert(10)->insert(1)->insert(6)
		->insert(14)->insert(4)->insert(7)->insert(13)->insert(19);
	*/
	for (int i=0; i<vec.size(); i++) {
		tree->insert(vec[i]);
	}

	// tree->levelsOrder();
	cout << "================================" << endl;
	tree->printOrder();

	cout << "==> Search 7: " << tree->search(7) << endl;
	cout << "==> Search 5: " << tree->search(5) << endl;

	/*
	int val;
	cout << "==> Enter node to delete: ";
	cin >> val;
	*/

	Node<int>* removed = tree->remove(val);
	cout << "==> Removed: " << val  << ", " << removed << endl;

	tree->printOrder();

	cout << "==> DELETE Tree" << endl;
	delete tree;
}

int main() {
	vector<int> v1 = {8, 3, 10, 1, 6, 14, 4, 7, 13, 19};
	test01(v1, 13);
	test01(v1, 8);

	vector<int> v2 = {8, 10, 14};
	test01(v2, 8);
	test01(v2, 10);

	vector<int> v3 = {8};
	test01(v3, 8);

	vector<int> v4 = {8, 3};
	test01(v4, 8);

	return 0;
}

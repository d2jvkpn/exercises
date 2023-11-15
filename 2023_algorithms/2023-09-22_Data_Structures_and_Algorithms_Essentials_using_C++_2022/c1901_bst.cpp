# include <iostream>
# include <vector>
# include "lib/c19_binary_search_tree.h"

using namespace std;

void test01(vector<int>& vec, int val) {
	// cout << "Hello, world!" << endl;
	Tree<int>* tree = new Tree<int>();

	/*
	tree->insert(8)->insert(3)->insert(10)->insert(1)->insert(6)
		->insert(14)->insert(4)->insert(7)->insert(13)->insert(19);
	*/

	/*
	for (int i=0; i<vec.size(); i++) {
		tree->insert(vec[i]);
	}
	*/

	tree->insert_vec(vec);

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

	bool removed = tree->remove(val);
	cout << "==> Removed: " << val  << ", " << removed << endl;

	tree->printOrder();

	cout << "==> DELETE Tree" << endl;
	delete tree;
}

int main() {
	Tree<int>* t0 = new Tree(42);
	t0->printOrder();

	vector<int> v1 = {8, 3, 10, 1, 6, 14, 4, 7, 13, 19};

	Tree<int>* t1 = new Tree<int>();
	t1->insert_vec(v1);
	cout << "==> t1: printRange(10, 20): ";
	t1->printRange(10, 20);

	test01(v1, 13);
	test01(v1, 8);

	vector<int> v2 = {8, 10, 14};
	test01(v2, 8);
	test01(v2, 10);

	vector<int> v3 = {8};
	test01(v3, 8);

	vector<int> v4 = {8, 3};
	test01(v4, 8);

	cout << "================" << endl;
	vector<int> v5 = {8};
	Tree<int>* t5 = new Tree<int>();
	t5->insert_vec(v5);
	t5->remove(8);
	if (t5->root == NULL) {
		cout << "!!! t5.root is NULL" << endl;
	}
	// cout << "==> t5.root: " << t5->root->data << endl;

	cout << "================" << endl;
	Tree<int>* t6 = new Tree<int>();
	t6->insert_vec(v5);
	Node<int>* root = t6->root;
	delete root;
	root = NULL;
	if (t6->root != NULL) {
		cout << "!!! t6.root isn't NULL" << endl;
	}
	cout << "==> t6.root: " << t6->root->data << endl; // indeterminate value"

	return 0;
}

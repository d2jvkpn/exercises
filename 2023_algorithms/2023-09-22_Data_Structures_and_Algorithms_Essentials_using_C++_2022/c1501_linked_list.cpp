#include <iostream>
#include "./lib/linked_list.h"

using namespace std;


int main() {
	List<int> list;
	list.show();

	list.push_back(42);
	list.show();

	list.push_back(101);
	list.show();

	list.push_back(102);
	list.push_back(103);
	list.push_front(41);
	list.show();

	list.insert(199, 2);
	list.show();

	cout << "==> Search 33: " << list.search(33) << endl;
	cout << "==> Search 199: " << list.search(199) << endl;

	list.show();
	list.reverse();
	list.show();

	Node<int>* ans1 = list.pop_back();
	cout << "==> Pop Back: " << ans1->data << endl;
	delete ans1;
	list.show();

	Node<int>* ans2 = list.pop_front();
	cout << "==> Pop Front: " << ans2->data << endl;
	delete ans2;
	list.show();

	return 0;
}

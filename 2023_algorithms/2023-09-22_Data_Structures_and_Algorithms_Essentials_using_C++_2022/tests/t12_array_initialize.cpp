#include <iostream>

using namespace std;

int main() {
	int num = 3;
	int** arr1 = new int*[num];

	cout << "==> arr1: ";
	for (int i=0; i<num; i++) {
		if (arr1[i] == NULL) {
			cout << "yes " << i << ", ";
		}
	}
	cout << endl;

	int arr2[5] = { 1, 2 };
	cout << "==> arr2: ";
	for (int i=0; i<5; i++) {
		cout << arr2[i] << ", ";
	}
	cout << endl;

	return 0;
}

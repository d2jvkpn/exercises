#include <iostream>
#include <vector>
#include "help.h"

using namespace std;

int main() {
	cout << "Hello, world!\n";

	int arr1[] = {1, 2, 3, 4, 5};
	int s1 = sizeof(arr1)/sizeof(int);
	showArray(arr1, s1);

	string arr2[] = {"Hello", "world"};
	int s2 = sizeof(arr2)/sizeof(string);
	showArray(arr2, s2);

	vector<int> vec1;
	vec1.reserve(5);

	vec1.push_back(1);
	vec1.push_back(2);
	vec1.push_back(3);

	showArray(vec1);

	return 0;
}

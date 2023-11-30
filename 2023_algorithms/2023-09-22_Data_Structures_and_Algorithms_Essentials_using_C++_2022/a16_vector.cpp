#include <iostream>
#include "include/vector.h"

using namespace std;

int main() {
	// cout << "Hello, world!\n";
	Vector<int> vec;

	vec.push_back(1);
	vec.push_back(2);
	vec.push_back(3);
	vec.push_back(4);
	vec.push_back(5);

	cout << "==> vec[3]: " << vec[3] << endl;
	cout << "==> vec: " << vec.size() << ", " << vec.capacity() << endl;

	Vector vec2(10, 42);
	vec2.print();

	return 0;
}

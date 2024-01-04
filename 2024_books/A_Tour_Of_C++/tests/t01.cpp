#include <iostream>
#include <vector>

using namespace std;

int main() {
	// cout << "Hello, world!\n";

	vector<int32_t> v1;
	v1.reserve(5);

	v1.push_back(42);
	cout << "v1[1]: " << v1[1] << endl; // OUTPUT: 0

	return 0;
}

#include <iostream>
#include <vector>

using namespace std;

int main() {
	cout << "Hello, world!\n";

	vector<int> v1;
	// cout << v1[1] << endl; // Segmentation fault
	v1.push_back(42);

	printf("==> size(v1)=%ld, capacity(v1)=%ld, v1[0]=%d, v1[1]=%d\n",
	  v1.size(), v1.capacity(), v1[0], v1[1]);

	vector<int> v2;
	v2.reserve(3);
	cout << "==> v2[0]: " << v2[0] << endl;

	vector<int> v3;
	cout << "==> v3[0]: " << v3[0] << endl;

	return 0;
}

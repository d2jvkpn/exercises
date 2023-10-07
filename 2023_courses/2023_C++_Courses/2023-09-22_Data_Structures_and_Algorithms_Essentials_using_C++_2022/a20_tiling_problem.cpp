#include <iostream>

using namespace std;

int tiling(int num) {
	if (num <= 0) {
		return 0;
	}

	if (num < 4) {
		return 1;
	}

	return tiling(num - 1) + tiling(num - 4);
}

int main() {
	// cout << "Hello, world!" << endl;

	int num;
	cout << "==> Tiling number: ";
	cin >> num;
	cout << "ans: " << tiling(num) << endl;

	return 0;
}

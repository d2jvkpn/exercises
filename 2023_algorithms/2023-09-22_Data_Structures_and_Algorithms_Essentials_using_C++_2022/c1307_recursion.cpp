#include <iostream>

using namespace std;

// O(n)
int power(int b, int n) {
	int ans = 1;

	for (int i=1; i<=n; i++) {
		ans *= b;
	}

	return ans;
}

// O(n)
int powerV1(int b, int n) {
	if (n==0) {
		return 1;
	}

	return b*powerV1(b, n-1);
}

// O(log(n))
int powerV2(int b, int n) {
	if (n==0) {
		return 1;
	}

	int ans = powerV2(b, n/2) * powerV2(b, n/2);

	if (n%2==1) {
		ans *= b;
	}

	return ans;
}

int main() {
	cout << "Hello, world!\n";

	return 0;
}

#include <iostream>

using namespace std;

int fib(int n) {
	if (n < 2) {
		return n;
	}

	return fib(n-1) + fib(n-2);
}

int fib2(int n) {
	int dp[n+1] = {0};
	dp[0] = 0;
	dp[1] = 0;

	for (int i=2; i<=n; i++) {
		dp[i] = dp[i-1] + dp[i-2];
	}

	return dp[n];
}

int fib3(int n) {
	if (n < 2) {
		return n;
	}

	int v1 = 0, v2 = 1;

	for (int i=2; i<=n; i++) {
		v2 = v1 + v2;
		v1 = v2 - v1;
	}

	return v2;
}

int main() {
	// cout << "Hello, world!\n";
	cout << "==> Enter fibnacci: ";

	int val;
	cin >> val;

	if (cin.fail()) {
		cerr << "!!! invalid input" << endl;
		exit(1);
	}

	cout << "ans: " << fib3(val) << endl;

	return 0;
}

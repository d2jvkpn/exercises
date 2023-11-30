#include <iostream>
#include <vector>

using namespace std;

template <typename A, typename B>
using Pair = pair<A, B>;

template <typename T>
using Vector = vector<T>;

// Top down: recursive code
double knapsackRecur(Pair<double, double> goods[], int size, double maxWt) {
	if (size == 0 || maxWt == 0.0) {
		return 0.0;
	}
	// cout << "~ size: " << size << endl;

	double inc=0.0, exc=0.0; // include, exclude
	double wt = goods[size-1].first;
	double price = goods[size-1].second;

	if (wt <= maxWt) {
		inc = price + knapsackRecur(goods, size-1, maxWt - wt);
	}
	exc = knapsackRecur(goods, size-1, maxWt);

	return max(inc, exc);
}

// Bottom up: DP
int knapsackDP(Pair<int, int> goods[], int size, int maxWt) {
	if (size == 0 || maxWt == 0.0) {
		return 0;
	}

	Vector<Vector<int>> dp(size+1, Vector<int>(maxWt+1, 0));

	for (int n=1; n<=size; n++) {
		for (int w=1; w<=maxWt; w++) {
			int wt = goods[n-1].first;
			int price = goods[n-1].second;

			int inc = wt <= w ? price + dp[n-1][w-wt] : 0;
			int exc = dp[n-1][w];

			dp[n][w] = max(inc, exc);
		}
	}

	return dp[size][maxWt];
}

int main() {
	// cout << "Hello, world!\n";
	int size;

	// Pair<Weight double, Price double>
	// Vector<Pair<double, double>> goods{{2.0, 5.0}, {7.0, 20.0}, {3.0, 20.0}, {4.0, 10.0}};
	Pair<double, double> goods1[] = {{2.0, 5.0}, {7.0, 20.0}, {3.0, 20.0}, {4.0, 10.0}};
	size = sizeof(goods1)/sizeof(Pair<double, double>);

	cout << "==> knapsackRecur ans1: " << knapsackRecur(goods1, size, 7.0) << endl;
	cout << "==> knapsackRecur ans2: " << knapsackRecur(goods1, size, 11.0) << endl;

	//
	Pair<int, int> goods2[] = {{2, 5}, {7, 20}, {3, 20}, {4, 10}};
	size = sizeof(goods2)/sizeof(Pair<int, int>);

	cout << "==> knapsackDP ans1: " << knapsackDP(goods2, size, 7) << endl;
	cout << "==> knapsackDP ans2: " << knapsackDP(goods2, size, 11) << endl;

	return 0;
}

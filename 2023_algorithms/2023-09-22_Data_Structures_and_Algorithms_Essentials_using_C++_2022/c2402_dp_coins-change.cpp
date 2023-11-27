# include <iostream>
# include <vector>
# include <climits>

using namespace std;

// Bottom Up DP
int minNumberOfCoinsForChange(int target, vector<int> denoms) {
	vector<int> dp(target+1, 0);
	dp[0] = 0;
	int prev;

	for (int index=1; index<=target; index++) {
		dp[index] = INT_MAX;

		for (int value: denoms) {
			prev = index - value;
			/*
			if (prev >= 0 && dp[prev] != INT_MAX) {
				dp[index] = min(dp[index], dp[prev] + 1);
			}
			*/
			if (prev >= 0 && dp[index] > dp[prev] + 1) {
				dp[index] = dp[prev] + 1;
			}
		}
	}

	return (dp[target] == INT_MAX) ? -1 : dp[target];
}

int main() {
	// vector<int> denoms = {1, 5, 7, 10};
	// int target = 8;
	vector<int> denoms = {1, 5, 10, 20, 50};
	int target;

	cout << "~~~ denoms: ";
	for (int v: denoms) {
		cout << v << ", ";
	}
	cout << endl;

	cout << "==> Enter amount: ";
	cin >> target;

	cout << minNumberOfCoinsForChange(target, denoms) << endl;

	return 0;
}

#include <iostream>
#include <vector>

using namespace std;

template <typename T>
using Vector = vector<T>;

template <typename T>
int lis(Vector<T> &vec) {
	int size = vec.size(), ans = 1;
	Vector<int> dp(size, 1);

	for (int i=1; i<size; i++) {
		for (int j=0; j<i; j++) {
			if (vec[i] > vec[j]) {
				dp[i] = max(dp[i], dp[j]+1);
				ans = max(ans, dp[i]);
			}
		}
	}

	return ans;
}

int main() {
	// cout << "Hello, world!\n";
	Vector<int> vec{50, 4, 10, 8, 30, 100};

	cout << "ans: " << lis(vec) << endl;

	return 0;
}

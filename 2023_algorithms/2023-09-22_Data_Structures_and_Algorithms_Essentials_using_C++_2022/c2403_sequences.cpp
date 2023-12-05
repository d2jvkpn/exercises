#include <iomanip>
#include <iostream>
#include <vector>

#include "include/help.h"

using namespace std;

template <typename T>
using Vector = vector<T>;

// longest increasing subsequence
template <typename T>
int longestIncreasingSubsequence(Vector<T> &vec) {
	int size = vec.size(), ans = 1;
	Vector<int> dp(size, 1);

	cout << "==> Longest Increasing Subsequence: " << vectorToString(vec) << endl;

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

string longestCommonSubstring(string &s1, string &s2) {
	int rows = s1.length(), cols = s2.length();

	if (rows < cols) {
		return longestCommonSubstring(s2, s1);
	}

	cout << "==> Longest Increasing Subsequence: " << quoted(s1) << ", " << quoted(s2) << endl;

	int max_length = 0, index = 0;
	Vector<int> current(cols+1, 0);
	Vector<int> dp(cols+1, 0);

	for (int i=1; i<rows; i++) {
		for (int j=1; j<cols; j++) {
			if (s1[i-1] != s2[j-1]) {
				 continue;
			}

			current[j] = dp[j - 1] + 1;

			if (current[j] > max_length) {
				max_length = current[j];
				index = i - max_length;
			}
		}

		for (int z=1; z<=cols; z++) {
			dp[z] = current[z];
			current[z] = 0;
		}
	}

	if (max_length > 0) {
	   return s1.substr(index, max_length);
	} else {
		return "";
	}
}

string longestCommonSubsequence(string &s1, string &s2) {
	int rows = s1.length(), cols = s2.length();

	if (rows < cols) {
		return longestCommonSubsequence(s2, s1);
	}

	cout << "==> Longest Common Subsequence: " << quoted(s1) << ", " << quoted(s2) << endl;

	Vector<int> dp(cols+1, 0);
	Vector<int> current(cols+1, 0);

	for (int i=1; i<=rows; i++){
		for (int j=1; j<=cols; j++) {
			if (s1[i-1] == s2[j-1]) {
				current[j] = dp[j-1] + 1;
			} else {
				current[j] = max(dp[j], current[j-1]);
			}
		}

		for (int z=1; z<=cols; z++) {
			dp[z] = current[z];
			current[z] = 0;
		}
	}

	// showVector(dp);
	// return dp[cols-1];
	stringstream ss;

	for (int i=1; i<=cols; i++) {
		if (dp[i-1] != dp[i]) {
			ss << s2[i-1];
		}
	}

	return ss.str();
}

int main() {
	// cout << "Hello, world!\n";

	// string t1 = "\"ac";
	// cout << t1 << ", " << quoted(t1) << endl; // "ac, "\"ac"

	//
	Vector<int> vec{50, 4, 10, 8, 30, 100};
	int ans = longestIncreasingSubsequence(vec);
	cout << "ans: " << ans << endl;

	//
	string s1 = "abcdxyz";
	string s2 = "xyzabcdki";
	string ans1 = longestCommonSubstring(s1, s2);
	cout << "ans: " << quoted(ans1) << endl;

	//
	s1 = "ADBCEFKKK";
	s2 = "ABCFGH";

	string ans2 = longestCommonSubsequence(s1, s2);
	cout << "ans: " << quoted(ans2) << endl;
}

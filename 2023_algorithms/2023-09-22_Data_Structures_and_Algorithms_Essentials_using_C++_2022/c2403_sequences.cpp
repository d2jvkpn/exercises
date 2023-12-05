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

    int max_length = 0, index = 0;
    Vector<int> current(cols+1, 0);
    Vector<int> dp(cols+1, 0);

    for (int i=1; i<=rows; i++) {
        for (int j=1; j<=cols; j++) {
            if (s1[i-1] != s2[j-1]) {
                 continue;
            }

            current[j] = dp[j - 1] + 1;

            if (current[j] > max_length) {
                max_length = current[j];
                index = i - max_length;
            }
        }

		for (int z=0; z<=cols; z++) {
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

int longestCommonSubsequence(string &s1, string &s2) {
	int rows = s1.length(), cols = s2.length();

	if (rows < cols) {
		return longestCommonSubsequence(s2, s1);
	}

    Vector<int> dp(cols, 0);
    Vector<int> current(cols, 0);

    for (int i=0; i<rows; i++){
        for (int j=0; j<cols; j++) {
            if (s1[i] == s2[j]) {
                current[j + 1] = dp[j] + 1;
            } else {
                current[j + 1] = max(dp[j + 1], current[j]);
            }
        }

		for (int z=0; z<=cols; z++) {
		    dp[z] = current[z];
		    current[z] = 0;
		}
    }

    return dp[cols];
}

int main() {
	// cout << "Hello, world!\n";
	//
	Vector<int> vec{50, 4, 10, 8, 30, 100};
	showVector(vec);

	cout << "==> longest_increasing_subsequence ans: " << longestIncreasingSubsequence(vec) << endl;

	//
	string s1 = "abcdxyz";
	string s2 = "xyzabcdki";
	cout << "==> longest_common_substring ans: " << longestCommonSubstring(s1, s2) << endl;

	//
	s1 = "ADBC";
    s2 = "ABC";

    cout << "==> longestCommonSubsequence: " << longestCommonSubsequence(s1, s2) << endl;
}

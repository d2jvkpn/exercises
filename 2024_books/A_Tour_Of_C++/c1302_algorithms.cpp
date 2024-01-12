#include <iostream>
#include <algorithm>

using namespace std;

bool has_c(const string& s, char c) {
	auto p = find(s.begin(), s.end(), c);
	return p != s.end();
}

vector<char> find_all(const string& s, char c) {
	vector<char> res;

	for (auto p = s.begin(); p!=s.end(); p++) {
		if (*p == c) {
			res.push_back(*p);
		}
	}

	return res;
}

int main() {
	// cout << "Hello, world!\n";

	cout << "==> has_c(\"hello\", 'c'): " << has_c("hello", 'o') << endl;

	auto ans = find_all("hello, world", 'o');
	cout << "==> find_all(\"hello, world\", 'o'): " << ans.size() << endl;

	vector<int> a01 = {3, 5, 7, 1, 2};

	for (auto v : a01) {
		cout << v << ", ";
	}
	cout << "\n";

	sort(a01.begin(), a01.end());
	for (auto v : a01) {
		cout << v << ", ";
	}
	cout << "\n";

	return 0;
}

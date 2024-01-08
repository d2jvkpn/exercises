#include <iostream>
#include <vector>

using namespace std;

vector<int> read_ints(istream& ist, const string& terminator) {
	vector<int> res;

	for (int i; ist >> i; ) {
		res.push_back(i);
	}

	if (ist.eof()) {
		// fine: end of file
		return res;
	}

	if (ist.fail()) {
		// we failed to read an int; was it the terminator?
		ist.clear();
		// reset the state to good()
		string s;

		if (ist>>s && s == terminator) {
			return res;
		}

		ist.setstate(ios_base::failbit);
		// add fail() to is’s state
	}

	return res;
}

int main() {
	// cout << "Hello, world!\n";

	auto v = read_ints(cin, "stop");

	if (cin.fail()) {
		cout << "!!! Failed" << endl;
	}

	for (int i=0; i<v.size(); i++) {
		cout << "~~~ " << v[i] << endl;
	}

	return 0;
}

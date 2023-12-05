# include <iostream>

using namespace std;

int main() {
	string s1 = "Hello";

	for (char c: s1) {
		cout << "char: " << c << endl;
	}

	cout << s1.substr(0, 2) << endl;
	cout << s1.substr(0, 0) << endl;

	return 0;
}

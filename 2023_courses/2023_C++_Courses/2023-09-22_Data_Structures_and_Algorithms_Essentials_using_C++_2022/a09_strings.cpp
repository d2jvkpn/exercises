# include <iostream>
# include <cstring>
# include <vector>

using namespace std;

int main() {
	// char s[100] = {'H', 'e', 'l', 'l', 'o', '\0'};
	string str; // s = "Hello, world"; // Dynamic Array

	cout << "Enter string ends with ." << endl;
	getline(cin, str, '.');

	for (char c: str) {
		cout << c << ", ";
	}

	cout << "\b\b." << endl;

	vector<string> array;
	string temp;
	int n = 5;

	cin.ignore();

	while (n > 0) {
		cout << "Enter a line: ";
		getline(cin, temp);
		array.push_back(temp);
		n--;
	}

	for (string s: array) {
		cout << s << endl;
	}

	return 0;
}

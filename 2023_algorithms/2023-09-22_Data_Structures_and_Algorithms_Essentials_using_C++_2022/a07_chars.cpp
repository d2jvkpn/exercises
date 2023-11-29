#include <iostream>

using namespace std;

int main() {
	char ch;
	int alphabets = 0, spaces = 0, digits = 0;

	while (true) {
		ch = cin.get();
		if (ch == '\n') {
			break;
		}

		if (ch >= '0' && ch <= '9') {
			digits++;
		} else if ((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')) {
			alphabets++;
		} else if (ch == ' ' || ch == '\t') {
			spaces++;
		}
	}

	cout << "==> Alphabets: " << alphabets << ", Digits: " << digits <<
		", Spaces: " << spaces << ".\n";
}

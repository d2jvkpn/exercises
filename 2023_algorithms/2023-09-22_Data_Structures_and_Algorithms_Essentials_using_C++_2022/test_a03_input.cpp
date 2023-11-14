# include <iostream>
# include <limits>

using namespace std;

int main() {
	int input;

	while (true) {
		cout << "==> Enter node integer: ";
		cin >> input;

		if (!cin.fail()) {
			break;
		}
		cout << "!!! Invalid integer" << endl;

		cin.clear();
		cin.ignore(numeric_limits<streamsize>::max(), '\n');
	}
}

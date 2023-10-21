# include <iostream>

using namespace std;

int pairing(int num) {
	if (num <= 1) {
		return 1;
	} else if (num == 2) {
		return 2;
	}

	return pairing(num - 1) + pairing(num - 2);
}

int main() {
	int num;

	cout << "Number of firend(s): ";
	cin >> num;

	cout << "ans: " << pairing(num) << endl;

	return 0;
}

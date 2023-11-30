#include <iostream>

using namespace std;

int main() {
	int age;
	cout << "Enter your age: ";
	cin >> age;

	try {
		if (age >= 18) {
			cout << "Eligible for voting." << endl;
		} else {
			throw(age);
		}
	}

	catch(int age) {
		cout << "Not eligible for voting, age = " << age << "!" << endl;
	}

	return 0;
}

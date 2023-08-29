# include <iostream>

using namespace std;

int main() {
	int num = 0;

	cout << "Please input your numher:\n";
	cin >> num;

	if (num < 10) {
		cout << "Your number is less than 10!\n";
	} else if (num >= 10 && num < 15) {
		cout << "Your number is greater or equal than 10 and below 15!\n";
	} else {
		cout << "Your number is greater or equal than 15!\n";
	}
}

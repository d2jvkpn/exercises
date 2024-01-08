#include <iostream>

using namespace std;

int main() {
	// cout << "Hello, world!\n";

	//
	char c;
	cin >> c;
	cout << "Got: " << c << endl;

	//
	constexpr double pi = 3.14159;
	cout << pi << "; "
		<< scientific << pi << "; "
		<< hexfloat << pi << "; "
		<< fixed << pi << "; "
		<< defaultfloat << pi << '\n';

	cout.precision(3);
	cout << "precision(3): " << pi << endl;

	//
	printf("an int %g and a string '%s'\n",12e3,"Hello!");

	return 0;
}

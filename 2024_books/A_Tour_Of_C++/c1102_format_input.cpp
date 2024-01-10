#include <iostream>
#include <fmt>

using namespace std;

int main() {
	// cout << "Hello, world!\n";

	//
	char c;
	cout << "==> Enter a char: ";
	cin >> c;
	cout << "==> Got: " << c << endl;

	//
	constexpr double pi = 3.14159;
	cout << "==> Pi: " << pi << "; "
		<< scientific << pi << "; "
		<< hexfloat << pi << "; "
		<< fixed << pi << "; "
		<< defaultfloat << pi << '\n';

	cout.precision(3);
	cout << "==> Precision(3): Pi=" << pi << endl;

	//
	printf("==> An int %g and a string '%s'\n", 12e3, "Hello!");
	cout << format("%s", "a") << endl;

	return 0;
}

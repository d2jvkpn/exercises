# include <iostream>

using namespace std;

double square(double);

void print_square(double x) {
	cout << "The square of " << x << " is " << square(x) << "\n";
}

int main() {
	// cout << "Hello, world!\n";
	print_square(1.234);

	return 0;
}

double square(double x) {
	return x*x;
}

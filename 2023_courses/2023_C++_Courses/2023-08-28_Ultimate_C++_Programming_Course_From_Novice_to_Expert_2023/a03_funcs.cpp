#include <iostream>

using namespace std;

int sum(int a, int b, int c = 2) {
	return a + b + c;
}

int my_func(int a) {
    return a+1;
}

double my_func(double a) {
    return a+1.0;
}

int main() {
	cout << sum(1,3,5) << endl;

	cout << sum(1, 3) << endl;

	cout << my_func(1) << endl;
	cout << my_func(2.1) << endl;

	return 0;
}

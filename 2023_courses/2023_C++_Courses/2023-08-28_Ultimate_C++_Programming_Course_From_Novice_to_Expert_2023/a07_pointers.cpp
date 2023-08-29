# include <iostream>

using namespace std;

int main() {
	int a = 42;
	int *p = &a;

	cout << p << endl;
	cout << *p << endl;

	*p += 1;
	cout << a << endl;

	int* x = &a;
	cout << (x == p) << endl;

	return 0;
}

# include <iostream>

using namespace std;

double square(double);

void print_square(double x) {
	cout << "The square of " << x << " is " << square(x) << "\n";
}

// count the number of occurrences of x in p[]
// p is assumed to point to a zero-terminated array of char (or to nothing)
int count_x(const char* p, char x) {
	if (p==nullptr) {
		return 0;
	}

	int count = 0;
	for (; *p!=0; p++) {
		if (*p==x) {
			++count;
		}
	}
	return count;
}

bool accept() {
	char ans;

	while (true) {
		cout << "==> Do you want to process(y/n)? ";

		cin >> ans;

		switch (ans) {
		case 'y':
			return true;
		case 'n':
			return false;
		default:
			return false;
		}
	}
}

int main() {
	// cout << "Hello, world!\n";
	print_square(1.234);

	//
	auto ans = accept();

	cout << "ans: " << ans << endl;

	//
	int x=2, y=3;
	int &a = x;
	int &b = y;

	cout << a << ", " << b << endl; // 2, 3

	b = a;
	cout << a << ", " << b << endl; // 2, 2
	cout << x << ", " << y << endl; // 2, 2

	// the task of initialization is to make an uninitialized piece of memory into a valid object.
	int v1 = 7;
	int& r1 {v1};

	cout << (r1==7) << endl;
	v1 = 42;
	cout << (r1==42) << endl;

	/*
	int& r2; // we cannot have an uninitialized reference
	r2 = 99;
	cout << r2 << endl;
	*/

	return 0;
}

double square(double x) {
	return x*x;
}

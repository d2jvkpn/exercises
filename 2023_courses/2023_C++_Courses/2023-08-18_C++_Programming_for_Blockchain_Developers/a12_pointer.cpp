#include <iostream>

using namespace std;

void increaseValue(int a) {
	a+=1;
	cout << "~~~ [increaseValue] Increase value to: " << a << endl;
}

void increasePointer(int& a) {
	a+=1;
	cout << "~~~ [increasePointer] Increase pointer to: " << a << endl;
}

void increasePtr(int *a) {
	*a+=1;
	cout << "~~~ [increasePtr] Increase pointer to: " << *a << endl;
}

int main() {
	//
	int a = 1;
	increaseValue(a);
	cout << "==> Value of a: " << a << endl;

	increasePointer(a);
	cout << "==> Value of a: " << a << endl;

	//
	int myAge = 21;
	cout << "==> My age is " << myAge << endl;
	cout << "==> The address of myAge is " << &myAge << endl;

	int* ptr = &myAge;
	cout << "==> Pointer points to: " << ptr << endl;
	cout << "==> The pointer points to: " << *ptr << endl;

	*ptr += 1;
	increasePtr(ptr);
	cout << "==> After ptr increase increase: " << myAge << endl;

	return 0;
}

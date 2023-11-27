# include <iostream>

using namespace std;

int main() {
	cout << "Hello, world!\n";

	int arr[3] = {7, 8, 9};

	cout << arr << ", " << arr + 1 << ", " << arr + 2 << endl;
	cout << &arr[0] << ", " << arr[1] << endl;

	cout << &arr[4] << ", " << arr[4] << endl;

	return 0;
}

# include <iostream>

using namespace std;

void new_array(const int size) {
	int array[size];
	array[0] = 3;
	cout << array[0] << endl;
	cout << array[1] << endl;
}

int main() {
	cout << "Hello, world!\n";
	new_array(10);
	new_array(0);

	return 0;
}

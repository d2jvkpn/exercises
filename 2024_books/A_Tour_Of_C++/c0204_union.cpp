#include <iostream>
#include <cstring>

using namespace std;

union Data {
   int i;
   char str[10];
};

int main() {
	Data data;

	data.i = 10;
	cout << data.i << ", " << data.str << endl;

	strcpy(data.str, "C++");
	cout << data.i << ", " << data.str << endl;

	data.i = 42;
	cout << data.i << ", " << data.str << endl;

	return 0;
}
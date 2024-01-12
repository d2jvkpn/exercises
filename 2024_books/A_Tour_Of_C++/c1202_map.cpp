#include <iostream>
#include <map>
#include <unordered_map>

using namespace std;

int main() {
	cout << "Hello, world!\n";

	map<string, int> pb1 = {
		{"David Hume", 123456},
		{"Karl Popper", 234567},
		{"Bertrand Arthur Willian Russell", 123456},
	};

	unordered_map<string, int> pb2 = {
		{"David Hume", 123456},
		{"Karl Popper", 234567},
		{"Bertrand Arthur Willian Russell", 123456},
	};

	cout << pb1["David Hume"] << ", " << pb2["Karl Popper"] << endl;

	return 0;
}

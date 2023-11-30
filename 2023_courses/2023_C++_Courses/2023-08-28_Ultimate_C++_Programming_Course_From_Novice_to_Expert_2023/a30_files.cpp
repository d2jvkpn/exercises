#include <iostream>
#include <fstream>

using namespace std;

int main() {
	//
	fstream f1;

	f1.open("target/hello.txt", ios::out);
	if (!f1) {
		cout << "==> File not created!" << endl;
		exit(1);
	}

	cout << "==> File created." << endl;
	f1 << "Hello";
	f1 << ", world!\n";
	f1.close();

	//
	fstream f2;
	char ch;

	f2.open("target/hello.txt", ios::in);
	if (!f2) {
		cout << "==> Can't open file!" << endl;
		exit(1);
	}

	cout << "'''";
	while (!f2.eof()) {
		f2 >> ch;
		cout << ch;
	}
	cout << "'''" << endl;
	f2.close();

	cout << "==> Exit" << endl;
	return 0;
}

# include <iostream>
# include <cstring>

using namespace std;

int main() {
	//
	char a1[10] = {'a', 'b', 'c'}; // wrong
	char a2[10] = {'a', 'b', 'c', '\0'}; // right
	char a3[10] = "abc";

	cout << "==> a1" << endl;
	cout << (a1[4] == '\0') << endl;
	cout << a1 << endl;
	for (int i=0; i<10; i++) {
		cout << a1[i] << ", ";
	}
	cout << endl;

	cout << "==> a2" << endl;
	cout << (a2[5] == '\0') << endl;
	cout << a2 << endl;
	for (int i=0; i<10; i++) {
		cout << a2[i] << ", ";
	}
	cout << endl;

	cout << "==> a3" << endl;
	cout << (a3[4] == '\0') << endl;
	cout << a3 << endl;
	for (int i=0; i<10; i++) {
		cout << a3[i] << ", ";
	}
	cout << endl;

	//
	cout << "==> a4" << endl;
	char a4[] = "abcd";
	cout << a4 << ", " << strlen(a4) << ", " << sizeof(a4) << endl;

	//
	cout << "==> a5, enter: ";
	char a5[100];
	cin >> a5;
	cout << a5 << ", " << strlen(a5) << ", " << sizeof(a5) << endl;

	return 0;
}

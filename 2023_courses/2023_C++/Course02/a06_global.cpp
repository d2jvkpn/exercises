# include <iostream>

using namespace std;

int glb = 42;

int main() {
	int glb = 5;

	cout << glb << endl;
	cout << ::glb << endl;

	return 0;
}

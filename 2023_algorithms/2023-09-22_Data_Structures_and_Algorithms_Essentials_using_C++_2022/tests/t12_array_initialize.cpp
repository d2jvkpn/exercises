#include <iostream>

using namespace std;

int main() {
	int num = 3;
	int** arr = new int*[num];

	for (int i=0; i<num; i++) {
		if (arr[i] == NULL) {
			cout << "yes: " << i << endl;
		}
	}

	return 0;
}

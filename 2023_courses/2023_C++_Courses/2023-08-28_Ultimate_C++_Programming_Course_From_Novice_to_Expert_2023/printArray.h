# include <iostream>

using namespace std;

void printArray(int array[], int size) {
	cout << "{ ";
	for (int i=0; i< size; i++) {
		if (i==0) {
			cout << array[i];
		} else {
			cout << ", " << array[i];
		}
	}

	cout << " }" << endl;
}

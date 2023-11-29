#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/*
void bubbleSort(int array[], int size) {
	for (int i=0; i < size - 1; i++) {
		for (int j=0; j < size - i - 1; j++) {
			if (array[j] > array[j+1]) {
				swap(array[j], array[j+1]);
			}
		}
	}
}
*/

void bubbleSort(int array[], int size) {
	for (int i=size-1; i>=0; i--) {
		for (int j=0; j < i; j++) {
			if (array[j] > array[j+1]) {
				swap(array[j], array[j+1]);
			}
		}
	}
}

void insertionSort(int array[], int size) {
	int val, prev;

	for (int i=1; i<size; i++) {
		val = array[i];

		for (prev=i-1; prev >= 0 && array[prev] > val; prev--) {
			array[prev + 1] = array[prev]; 
		}

		array[prev+1] = val;
	}
}

void selectionSort(int array[], int size) {
	for (int i=0; i<size; i++) {
		int pos = i;
		for (int j=i; j<size; j++) {
			if (array[pos] > array[j]) {
				pos = j;
			}
		}

		swap(array[i], array[pos]);
	}
}

bool greaterThan(int a, int b) {
	cout << "~~~ compare: " << a << ", " << b << endl;
	return a > b;
}

int main() {
	//
	int array1[] = {-2, 3, 4, -1, 5, -12, 6, 1, 3};
	int size1 = sizeof(array1)/sizeof(int);
	bubbleSort(array1, size1);

	cout << "==> array1: ";
	for (int i=0; i< size1; i++) {
		cout << array1[i] << ", ";
	}
	cout << endl;

	//
	int array2[] = {-2, 3, 4, -1, 5, -12, 6, 1, 3};
	int size2 = sizeof(array2)/sizeof(int);
	insertionSort(array2, size2);

	cout << "==> array2: ";
	for (int i=0; i< size2; i++) {
		cout << array2[i] << ", ";
	}
	cout << endl;

	//
	int array3[] = {-2, 3, 4, -1, 5, -12, 6, 1, 3};
	int size3 = sizeof(array3)/sizeof(int);
	selectionSort(array3, size3);

	cout << "==> array3: ";
	for (int i=0; i< size3; i++) {
		cout << array3[i] << ", ";
	}
	cout << endl;

	//
	int array4[] = {10, 9, 8, 6, 5, 4, 3, 2, 11, 16, 8};
	int size4 = sizeof(array4)/sizeof(int);
	sort(array4, array4 + size4);

	cout << "==> array4: ";
	for (int v: array4) {
		cout << v << ", ";
	}
	cout << endl;

	sort(array4, array4 + size4, greaterThan);
	// sort(array4, array4 + size4, greater<int>());
	cout << "==> array4: ";
	for (int v: array4) {
		cout << v << ", ";
	}
	cout << endl;

	cout << "==> array4: ";
	reverse(array4, array4 + size4);
	for (int v: array4) {
		cout << v << ", ";
	}
	cout << endl;

	return 0;
}

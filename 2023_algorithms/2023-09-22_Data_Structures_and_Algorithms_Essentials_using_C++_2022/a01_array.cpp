#include <iostream>

using namespace std;

void printArray(int arr[], int size) {
	cout << "==> { ";
	for (int i=0; i< size; i++) {
		cout << arr[i] << ", ";
	}
	cout << "\b\b }" << endl;
}

void printArray2(int* arr, int size) {
	cout << "==> { ";
	for (int i=0; i< size; i++) {
		cout << arr[i] << ", ";
	}
	cout << "\b\b }" << endl;
}

int main() {
	// int arr[100]; // size = 100, {undefined, undefined, undefined, ...}
	// int arr[100] = {1}; // size = 100, {1, 0, 0, ...}
	// int arr[100] = {0, 1, 2, 3}; // {0, 1, 2, 3, 0, 0, 0...}
	int arr[] = {0, 1, 2, 3}; // size = 4, {0, 1, 2, 3}

	int marks[100] = {0};
	int n;

	cout << "Enter the no of stdents: ";
	cin >> n;
	n = n > 100 ? 100 : n;

	for (int i = 0; i < n; i++) {
		cin >> marks[i];
	}

	cout << "Studnets: ";
	for (int i = 0; i < n; i++) {
		cout << marks[i] << ", ";
	}
	cout << endl;

	int arr[5] = {2};
	int size = sizeof(arr)/ sizeof(int);

	printArray(arr, size);

	return 0;
}

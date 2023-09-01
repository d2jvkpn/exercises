# include <iostream>

# include "helper.h"

using namespace std;

void selectionSort(int array[], int size) {
	if (size == 0) {
		return;
	}

	int idx, tmp;

	for (int i=0; i< size; i++) {
		idx = i;

		for (int j=i+1; j < size; j++) {
			if (array[j] < array[idx]) {
				idx = j;
			}
		}

		if (idx != i) {
			tmp = array[i];
			array[i] = array[idx];
			array[idx] = tmp;
		}
	}
}

void bubbleSort(int array[], int size) {
	if (size == 0) {
		return;
	}

	int tmp;

	for (int i=0; i < size; i++) {
		for (int j = 0; j < size-i-1; j++) {
			if (array[j] > array[j+1]) {
				tmp = array[j];
				array[j] = array[j+1];
				array[j+1] = tmp;
			}
		}
	}
}

void insertionSort(int array[], int size) {
	if (size == 0) {
		return;
	}

	int tmp, j;

	for (int i=1; i < size; i++) {
		tmp = array[i];
		j = i - 1;

		// array[0..j] is sorted
		while (j >= 0 && array[j] > tmp) {
			array[j+1] = array[j];
			j-=1;
		}

		array[j+1] = tmp;
	}
}

int main() {
	int array[] = {3, 8, 6, 0, 1, 4};
	int size = sizeof(array)/sizeof(int);

	int a1[] = {3, 8, 6, 0, 1, 4};
	selectionSort(a1, size);
	printArray(a1, size);

	int a2[] = {3, 8, 6, 0, 1, 4};
	bubbleSort(a2, size);
	printArray(a2, size);

	int a3[] = {3, 8, 6, 0, 1, 4};
	insertionSort(a3, size);
	printArray(a3, size);

	return 0;
}

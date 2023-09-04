# include <iostream>
# include <vector>

# include "helper.h"

using namespace std;

void merge(vector<int>& array, int left, int mid, int right) {
	int n1 = mid - left + 1;
	int n2 = right - mid;

	vector<int> leftArr(n1);
	vector<int> rightArr(n2);

	for (int i=0; i < n1; i++) {
		leftArr[i] = array[left + i];
	}

	for (int j=0; j < n2; j++) {
		rightArr[j] = array[mid + 1 + j];
	}

	int i = 0, j = 0, k = left;

	while(i < n1 && j < n2) {
		if (leftArr[i] <= rightArr[j]) {
			array[k] = leftArr[i];
			i+=1;
		} else {
			array[k] = rightArr[j];
			j+=1;
		}
		k+=1;
	}

	while(i < n1) {
		array[k] = leftArr[i];
		i+=1, k+=1;
	}

	while(j<n2) {
		array[k] = leftArr[j];
		j+=1, k+=1;
	}
}

void mergeSort(vector<int>& array, int left, int right) {
	if (left < right) {
		int mid = (left + right) / 2;
		mergeSort(array, left, mid);
		mergeSort(array, mid+1, right);
		merge(array, left, mid, right);
	}
}

int main() {
	vector<int> array = {3, 8, 6, 0, 1, 4};
	int size = array.size();

	mergeSort(array, 0, size - 1);
	printVector(array);

	return 0;
}

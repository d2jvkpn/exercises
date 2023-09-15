# include <iostream>

using namespace std;

int linear_search(int array[], int size, int target) {
	for (int i=0; i<size; i++) {
		if (array[i] == target) {
			return i;
		}
	}

	return -1;
}

int binary_search(int array[], int size, int target) {
	if (size == 0) {
		return -1;
	}

	int s = 0, e = size - 1, m;

	while (s <= e) {
		m = (s+e)/2;

		if (array[m] == target) {
			return m;
		} else if (array[m] > target) {
			e = m - 1;
		} else {
			s = m + 1;
		}
	}

	return -1;
}

int main() {
	// int array[5];
	int array[] = {1, 2, 10, 11, 19, 29, 38};
	int size = sizeof(array)/sizeof(int);

	int index = binary_search(array, size, 19);
	cout << "==> Found: " << index << endl;

	return 0;
}

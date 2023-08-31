# include <iostream>

using namespace std;

template <class T>
T binarySearch(T array[], T size, T target) {
	int left = 0, right = size, mid;

	while (left <= right) {
		mid = left + (right - left) / 2;
		if (array[mid] == target) {
			return mid;
		} else if (array[mid] < target) {
			left = mid +1;
		} else {
			right = mid - 1;
		}
	}

	return -1;
}

int main() {
	int array[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
	int size = sizeof(array) /sizeof(int);

	cout << binarySearch(array, size, 8) << endl; // 7

	return 0;
}

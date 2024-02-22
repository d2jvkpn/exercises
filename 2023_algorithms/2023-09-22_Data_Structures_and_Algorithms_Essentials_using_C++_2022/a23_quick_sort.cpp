#include <iostream>
#include <vector>

using namespace std;

int partition(vector<int> &vec, int low, int high) {
	if (low >= high) {
		return low;
	}

	int value = vec[high];
	int pivot = low;

	// .pivot...i...
	for (int i=low; i<=high; i++) {
		if (vec[i] < value) {
			swap(vec[pivot], vec[i]);
			pivot+=1; // vec[idx] >= value
		}
	}
	swap(vec[pivot], vec[high]);

	return pivot;
}

void quickSort(vector<int> &vec, int low, int high) {
	if (low >= high) {
		return;
	}

	int pivot = partition(vec, low, high);

	if (low + 1 < pivot) {
		quickSort(vec, low, pivot-1);
	}

	if (pivot+1 < high) {
		quickSort(vec, pivot+1, high);
	}
}

int main() {
	// cout << "Hello, world!\n";
	vector<int> vec = {10, 5, 2, 0, 7, 6, 4};
	
	quickSort(vec, 0, vec.size() - 1);

	for (int val: vec) {
		cout << val << ", ";
	}
	cout << endl;

	return 0;
}

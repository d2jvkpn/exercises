# include <iostream>
# include <vector>

using namespace std;

int partition(vector<int> &vec, int low, int high) {
	int pivot = vec[high];
	int index = low;

	// .index...i...
	for (int i=low; i<=high; i++) {
		if (vec[i] < pivot) {
			swap(vec[index], vec[i]);
			index+=1;  // vec[idx] >= pivot
		}
	}
	swap(vec[index], vec[high]);

	return index;
}

void quickSort(vector<int> &vec, int low, int high) {
	if (low >= high) {
		return;
	}

	int index = partition(vec, low, high);
	quickSort(vec, low, index-1);
	quickSort(vec, index+1, high);
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

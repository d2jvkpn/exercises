#include <iostream>
#include <vector>

using namespace std;

void merge(vector<int> &vec, int low, int high) {
	int i=low;
	int m = (low + high)/2;
	int j = m+1;
	cout << low << ", " << high << endl;

	vector<int> temp;
	temp.reserve(high - low + 1);

	while (i<=m and j <= high) {
		if (vec[i] < vec[j]) {
			temp.push_back(vec[i]);
			i+=1;
		} else {
			temp.push_back(vec[j]);
			j+=1;
		}
	}

	while (i<=m) {
		temp.push_back(vec[i]);
		i+=1;
	}

	while (j <= high) {
		temp.push_back(vec[j]);
		j+=1;
	}

	// copy(temp.begin(), temp.end(), back_inserter(vec));

	j=0;
	for (i=low; i<=high; i++) {
		vec[i] = temp[j];
		j+=1;
	}
}

void mergeSort(vector<int> &vec, int low, int high) {
	if (low >= high) {
		return;
	}

	int mid = (low + high) / 2;

	mergeSort(vec, low, mid);
	mergeSort(vec, mid+1, high);

	return merge(vec, low, high);
}

int main() {
	// cout << "Hello, world!" << endl;

	vector<int> vec{10, 5, 2, 0, 7, 6, 4};
	mergeSort(vec, 0, vec.size()-1);

	for (int val: vec) {
		cout << val << ", ";
	}
	cout << endl;

	return 0;
}

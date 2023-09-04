# include <iostream>
# include <vector>

# include "helper.h"

using namespace std;

void merge(vector<int>& vec, int l, int m, int r) {
	int  i = l, j = m+1;

	vector<int> temp;
	temp.reserve(r - l + 1);

	while (i<=m && j <= r) {
		if (vec[i] <= vec[j]) {
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

	while (j<=r) {
		temp.push_back(vec[j]);
		j+=1;
	}

	for(int p = 0; p< temp.size(); p++) {
		vec[p+l] = temp[p];
	}
}

void mergeSort(vector<int>& vec, int l, int r) {
	if(l < r) {
		int m = (l+r)/2;
		mergeSort(vec, l, m);
		mergeSort(vec, m+1, r);
		merge(vec, l, m, r);
	}
}

int main() {
	vector<int> vec = {3, 8, 6, 0, 1, 4};
	mergeSort(vec, 0, vec.size());

	printVector(vec);
}

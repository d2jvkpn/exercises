#include <iostream>
#include <vector>

using namespace std;

int partition(vector<int> &vec, int front, int end){
	int pivot = vec[end], i = front - 1;

	for(int j=front; j<end; j++){
		if(vec[j] < pivot){
			i++;
			swap(vec[i], vec[j]);
		}
	}

	swap(vec[i+1], vec[end]);
	return i + 1;
}

int quickSelect(vector<int> vec, int front, int end, int k){
	// assuming that k will be inside the array
	int pivot = partition(vec, front, end);

	if (pivot==k) {
		return vec[pivot];
	} else if (k < pivot) {
		return quickSelect(vec, front, pivot-1, k);
	} else {
		return quickSelect(vec, pivot+1, end, k);
	}
}

int main(){
    vector<int> vec{10, 5, 2, 0, 7, 6, 4};
    int k;


	for (auto v : vec) {
		cout << v << ", ";
	}
	cout << endl;

    cout << "==> Enter k: ";
    cin >> k;

    cout << quickSelect(vec, 0, vec.size()-1, k) << endl;

    return 0;
}

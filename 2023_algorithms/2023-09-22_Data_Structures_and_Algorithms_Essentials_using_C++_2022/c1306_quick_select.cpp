#include <iostream>
#include <vector>

using namespace std;

int partition(vector<int> &vec, int front, int end){
	int value = vec[end], pivot = front;

	for(int j=front; j<end; j++){
		if(vec[j] < value){
			swap(vec[pivot], vec[j]);
			pivot+=1;
		}
	}

	swap(vec[pivot], vec[end]);
	return pivot;
}

int quickSelectRecur(vector<int> &vec, int front, int end, int k){
	// assuming that k will be inside the array
	int pivot = partition(vec, front, end);

	if (pivot==k) {
		return vec[pivot];
	} else if (k < pivot) {
		return quickSelectRecur(vec, front, pivot-1, k);
	} else {
		return quickSelectRecur(vec, pivot+1, end, k);
	}
}

int quickSelect(vector<int> &vec, int k) {
	if (k >= vec.size() || k < 0) {
		throw out_of_range("k is out of range");
	}

	return quickSelectRecur(vec, 0, vec.size()-1, k);
}

int main(){
    vector<int> vec{10, 5, 2, 0, 7, 6, 4};
    int k;

	for (auto v : vec) {
		cout << v << ", ";
	}
	cout << endl;

	while (true) {
	    cout << "==> Enter k: ";
	    cin >> k;

		if (k<0) {
			break;
		}

	    cout << quickSelect(vec, k) << endl;
	}

    return 0;
}

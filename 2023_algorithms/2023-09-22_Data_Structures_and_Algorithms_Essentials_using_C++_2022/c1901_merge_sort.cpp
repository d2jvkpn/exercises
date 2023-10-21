#include<iostream>
#include <vector>

using namespace std;

//helper method
void merge(vector<int> &vec, int front, int end){
	int mid = (front + end)/2;
	int i = front, j = mid + 1;

	vector<int> temp;
	temp.reserve(end - front + 1);

	while (i <= mid || j <= end){
		if (j > end || vec[i] <= vec[j]){
			temp.push_back(vec[i]);
			i++;
		} else if (i > mid || vec[i] > vec[j] ){
			temp.push_back(vec[j]);
			j++;
		}
	}

	// copy back the eleemtns from temp to original vec
	int k = 0 ;
	for(int idx = front; idx <= end; idx++){
		vec[idx] = temp[k];
		k += 1;
	}

	return;
}

//sorting method
void mergeSort(vector<int> &vec, int front, int end){
	// base case
	if(front >= end){
		return;
	}

	// rec case
	int mid = (front + end)/2;
	mergeSort(vec, front, mid);
	mergeSort(vec, mid+1, end);
	return merge(vec, front, end);
}

int main(){
	vector<int> vec{10,5,2,0,7,6,4};

	mergeSort(vec, 0, vec.size() - 1);

	for (int x : vec){
		cout << x << ", ";
	}
	cout << endl;

	return 0;
}

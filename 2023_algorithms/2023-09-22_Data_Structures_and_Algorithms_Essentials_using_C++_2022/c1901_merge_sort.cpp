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
void mergeSort(vector<int> &arr, int s, int e){
	//base case
	if(s>=e){
		return;
	}

	//rec case
	int mid = (s+e)/2;
	mergeSort(arr,s,mid);
	mergeSort(arr,mid+1,e);
	return merge(arr,s,e);
}

int main(){
  	vector<int> arr{10,5,2,0,7,6,4};

  	mergeSort(arr, 0, arr.size() - 1);

	for (int x : arr){
		cout << x << ", ";
	}
	cout << endl;

	return 0;
}

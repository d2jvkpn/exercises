#include<iostream>
#include <vector>

using namespace std;

int partition(vector<int>& vec, int front, int end){
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

void quickSort(vector<int>& vec, int front, int end){
	// Base Case
	if(front >= end){
		return;
	}

	// Rec Case
	int pivot = partition(vec, front, end);
	quickSort(vec, front, pivot-1);
	quickSort(vec, pivot+1, end);
}


int main(){
	vector<int> vec{10,5,2,0,7,6,4};

	quickSort(vec, 0, vec.size()-1);

	for(int x : vec){
		cout << x << ", ";
	}
	cout << endl;

	return 0;
}

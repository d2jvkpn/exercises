#include<iostream>
using namespace std;

int binary_search(int arr[],int n,int key){
	int front = 0, end = n - 1, mid;

	while(front <= end){
		mid = (front + end)/2;

		if(arr[mid] == key){
			return mid;
		} else if(arr[mid] > key){
			end = mid - 1;
		} else{
			front = mid + 1;
		}
	}

	return -1;
}


int main(){
	int arr[] = {10,20,30,40,45,60,70,89};
	int size = sizeof(arr)/sizeof(int);
	int key;

	cout << "{ ";
	for (int i=0; i<size; i++) {
		cout << arr[i] << ", ";
	}
	cout << "\b\b }\n";

	cout << "==> Enter key: ";
	cin >> key;

	int index = binary_search(arr, size, key);
	if(index != -1){
		cout << key << " is present at index "<< index << ".\n";
	}
	else{
		cerr << key <<" is NOT Found!" << ".\n";
	}

	return 0;
}

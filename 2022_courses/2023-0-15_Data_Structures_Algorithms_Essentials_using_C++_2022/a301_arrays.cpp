# include <iostream>

using namespace std;

void print_array(int array[], int size) {	
	cout << "{ ";

	for (int i=0; i<size; i++) {
	    if (i!=0) {
	    	cout << ", ";
	    }
		cout << array[i];
	}

	cout << " }" << endl;
}

void reverse_array(int array[], int size) {
	if (size == 0) {
		return;
	}

	int s = 0, e = size - 1;

	while (s < e) {
		swap(array[s], array[e]);
		s+=1, e-=1;
	}
}

int main() {
	int array[5] = {1, 2, 3, 4, 5};
	int size = sizeof(array)/sizeof(int);

	print_array(array, size);
	reverse_array(array, size);
	print_array(array, size);

	return 0;
}

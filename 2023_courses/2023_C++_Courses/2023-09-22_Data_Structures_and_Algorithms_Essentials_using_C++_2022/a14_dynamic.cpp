# include <iostream>

using namespace std;

int** create2DArray(int rows, int cols) {
	int** array = new int*[rows];

	for (int r=0; r < rows; r++) {
		array[r] = new int[cols];
	}

	return array;
} 

void delete2DArray(int** array, int rows) {
	for (int i=0; i< rows; i++) {
		delete [] array[i];
	}

	delete [] array;
}

int main() {
	// cout << "Hello, world!\n";

	// job of the programmer ranther than the compiler, 
	int* array = new int[10];

	cout << array[0] << endl;
	array[0] = 42;
	cout << array[0] << endl;
	delete []array;

	int* val = new int;
	*val = 42;
	cout << *val << endl;

	//
	int n;
	cout << "==> Enter size: ";
	cin >> n;

	int* arr = new int[n];
	cout << "==> arr: " << arr << endl;
	
	for (int i=0; i<n; i++) {
		arr[i] = i;
	}

	cout << "==> fill values" << endl;
	for (int i=0; i<n; i++) {
		cout << arr[i] << " ";
	}
	cout << endl;
	delete [] arr;

	cout << "==> arr: " << arr << endl;

	cout << "==> after deleted" << endl;
	for (int i=0; i<n; i++) {
		cout << arr[i] << " ";
	}
	cout << endl;

	//
	int rows = 8, cols = 8;
	int** a = create2DArray(rows, cols);

	for (int r=0; r < rows; r++) {
		for (int c=0; c<cols; c++) {
			cout << a[r][c] << ", ";
		}
		cout << endl;
	}
	delete2DArray(a, rows);

	//
	int** myArray = (int**)malloc(rows * sizeof(int*));

	for(int i = 0; i < rows; ++i) {
	    myArray[i] = (int*)malloc(cols * sizeof(int));
	}

	for(int i = 0; i < rows; ++i) {
	    free(myArray[i]);
	}
	free(myArray);


	return 0;
}

# include <iostream>
# include <cmath>

using namespace std;

int prefix_sum(int array[], int size) {
	int prefix[size] = {0}, largest = 0, tmp = 0;

	for (int i=1; i<size; i++) {
		prefix[i] = prefix[i-1] + array[i];
	}

	for (int i=1; i<size; i++) {
		for (int j=i; j < size; j++) {
			tmp = i> 0 ? prefix[j] - prefix[i - 1] : prefix[j];
			largest = max(largest, tmp);
		}
	}

	return largest;
}

int kadane(int array[], int size) {
	int cs = 0, largest = 0;

	for (int i=0; i<size; i+=1) {
		cs += array[i];
		if (cs < 0) {
			cs = 0;
		}

		largest = max(largest, cs);
	}

	return largest;
}

int main() {
	int array[] = {-2, 3, 4, -1, 5, -12, 6, 1, 3};
	int size = sizeof(array) / sizeof(int);

	cout << prefix_sum(array, size) << endl;
	
	cout << kadane(array, size) << endl;

	return 0;
}

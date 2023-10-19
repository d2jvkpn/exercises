# include <iostream>

using namespace std;

void printArray(int *arr, int n) {
	cout << "{ ";
	for (int i=0; i<n; i++) {
			cout << arr[i] << ", ";
	}
	cout << "\b\b }" << endl;
}

void fillArray(int *arr, int n, int i, int val) {
	if (i == n) {
		printArray(arr, n);

		return;
	}

	arr[i] = val;
	fillArray(arr, n, i+1, val +1);

	// backtracking step
	arr[i] = -1 * arr[i];
}

int main() {
	int arr[100] = {0};
	int n;

	cout << "==> Enter n: ";
	cin >> n;
	n = (n > 100) ? 100 : n;

	fillArray(arr, n, 0, 1);
	printArray(arr, n);

	return 0;
}

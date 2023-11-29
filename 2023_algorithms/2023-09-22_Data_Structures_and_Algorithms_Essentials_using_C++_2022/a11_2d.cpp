#include <iostream>
#include <cstring>
#include <climits>

using namespace std;

int main() {
	int array[100][100];
	int n, m;

	cout << "Enter n and m: ";
	cin >> n;
	cin >> m;
	if (n>100) {
		n = 100;
	}
	if (m>100) {
		m = 100;
	}

	cout << "n: " << n << ", m: " << m << endl;
	// cin.clear();
	// cin.ignore(INT_MAX);

	cout << "==> Enter matrix:" << endl;
	for (int i=0; i<n; i++) {
		for (int j=0; j<m; j++) {
			cin >> array[i][j];
		}
	}

	// cout << "~~~ " << array[0][0] << endl;
	cout << "==> Matrix:" << endl;
	for (int i=0; i<n; i++) {
		for (int j=0; j<m; j++) {
			cout << array[i][j] << " ";
		}
		cout << endl;
	}

	int arr[4][4] = {
		{1, 2, 3, 4},
		{5, 6, 7, 8},
		{9, 10, 11, 12},
		{13, 14, 15, 16},
	};

	n = 4, m = 4;
	
}

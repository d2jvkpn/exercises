# include <iostream>

using namespace std;

int gridWays(int i, int j, int m, int n) {
	if (i>=m || j>=n) {
		return 0;
	}

	if (i==m-1 && j==n-1) {
		return 1;
	}

	return gridWays(i+1, j, m, n) + gridWays(i, j+1, m, n);
}

int main() {
	int m, n;
	cout << "==> Enter m and n: ";
	cin >> m;
	cin >> n;

	cout << "ans: " << gridWays(0, 0, m, n) << endl;

	return 0;
}

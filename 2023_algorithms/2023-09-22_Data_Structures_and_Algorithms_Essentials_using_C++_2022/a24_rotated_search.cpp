# include <iostream>
# include <vector>

using namespace std;

int rotatedSearch(vector<int> &vec, int target) {
	int size = vec.size();
	int low = 0, high = size - 1, mid;

	while (low <= high) {
		mid = (low+high)/2;
		printf("~~~ low=%d, mid=%d, high=%d\n", low, mid, high);

		if (vec[mid] == target) {
			return mid;
		}

		if (vec[low] <= vec[mid]) {
			if (target >= vec[low] and target <= vec[mid]) {
				high = mid - 1;
			} else {
				low = mid + 1;
			}
		} else {
			if  (target > vec[mid] and target <= vec[high]) {
				low = mid + 1;
			} else {
				high = mid - 1;
			}
		}
	}

	return -1;
}

int main() {
	// cout << "Hello, world!\n";

	vector<int> vec{4, 5, 6, 7, 0, 1, 2, 3};
	int target;

	printf("==> Enter target: ");
	cin >> target;

	printf("ans: %d\n", rotatedSearch(vec, target));

	return 0;
}

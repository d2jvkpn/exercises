#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

long long maximumPairwiseProduct(vector<int>& nums) {
	long long ans;
	int index1 = -1, index2 = -1;

	if (nums.size() <= 1) {
		return 0;
	}

	for (int i=0; i<nums.size(); i++) {
		if (index1 == -1 || nums[i] > nums[index1]) {
			index1 = i;
		}
	}

	for (int i=0; i<nums.size(); i++) {
		if (i == index1) {
			continue;
		}
		if (i == -1 || nums[i] > nums[index2]) {
			index2 = i;
		}
	}

	return ((long long) (nums[index1])) * ((long long) (nums[index2]));
}

int main() {
	srand (time(NULL));
	cout << "==> Random number: " << rand()%10 + 1 << endl;

	int size, val;

	cout << "==> Enter size of input: ";
	cin >> size;

	if (size < 2) {
		cerr << "!!! invalid size: " << size << endl;
		exit(1);
	}

	vector<int> nums(size);
	for (int i=0; i<size; i++) {
		cin >> nums[i];
	}

	cout << "==> Ans: " << maximumPairwiseProduct(nums) << endl;

	return 0;
}

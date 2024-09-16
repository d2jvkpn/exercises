#include <cstdlib>
#include <iostream>
#include <vector>

// using namespace std;

long long maximumPairwiseProduct(std::vector<int>& nums) {
	long long ans = 0;

	if (nums.size() <= 1) {
		return 0;
	}

	for (int i=0; i<nums.size(); i++) {
		for (int j=i+1; j<nums.size(); j++) {
			ans = std::max(
				ans,
				((long long) (nums[i])) * ((long long) (nums[j]))
			);
		}
	}

	return ans;
}

int main() {
	srand (time(NULL));
	std::cout << "==> Random number: " << rand()%10 + 1 << std::endl;

	int size, val;

	std::cout << "==> Enter size of input: ";
	std::cin >> size;

	if (size < 2) {
		std::cerr << "!!! invalid size: " << size << std::endl;
		std::exit(1);
	}

	std::vector<int> nums(size);
	for (int i=0; i<size; i++) {
		std::cin >> nums[i];
	}

	std::cout << "==> Ans: " << maximumPairwiseProduct(nums) << std::endl;

	return 0;
}

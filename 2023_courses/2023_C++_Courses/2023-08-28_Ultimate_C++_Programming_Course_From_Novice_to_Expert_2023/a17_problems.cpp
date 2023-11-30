#include <iostream>
#include <cmath>
#include <vector>
#include <climits>

using namespace std;

// pass by reference: vector<float>&
// pass by copy: vector<float>
float sd(vector<float>& data) {
	float sum = 0.0, mean, sd = 0.0;

	for (int i=0; i<data.size(); i++) {
		sum+=data[i];
	}
	mean = sum/data.size();

	for (int i=0; i< data.size(); i++) {
		sd += pow(data[i] - mean, 2);
	}

	data[0] = 42.0;
	return sqrt(sd/10.0);
}

int maxSumV1(vector<int>& vec) {
	 int ans = 0, cum=0, val=0, idx=0, size = vec.size();

	 for (int i=0; i < size; i++) {
		val = 0;
		for (int j=0; j < size; j++) {
			idx = (i+j) % size;
			val += j*vec[j];
		}

		ans = max(ans, val);
	 }

	return ans;
}

int maxSumV2(vector<int>& vec) {
	int cum = 0, val = 0, ans, next, size = vec.size();

	for (int i=0; i < size; i++) {
		cum += vec[i];
		val += i*vec[i];
	}

	ans = val;
	for (int i=1; i < size; i++) {
		next = val - (cum - vec[i-1] + vec[i-1]*(size-1));
		val = next;
		ans = max(ans, next);
	}

	return ans;
}

int minSwap(vector<int>& vec, int k) {
	int count = 0, bad = 0, ans, size = vec.size();

	for (int i=0; i<size; i++) {
		if (vec[i] <= k) {
			count += 1;
		} else {
			bad += 1;
		}
	}

	ans = bad;
	for (int i=0, j=count; j<size; i++, j++) {
		if (vec[i] > k) {
			bad -= 1;
		}

		if (vec[j] > k) {
			bad += 1;
		}

		ans = min(ans, bad);
	}

	return ans;
};

int main() {
	vector<int> vec = {1, 2, 3, 4, 5, 6, 7, 8, 9};

	cout << maxSumV1(vec) << endl;
	cout << maxSumV2(vec) << endl;

	// ?
	vector<int> v2 = {2, 1, 5, 6, 3};
	cout << minSwap(v2, 3) << endl;

	vector<int> v3 = {2, 7, 9, 5, 8, 7, 4};
	cout << minSwap(v3, 5) << endl;

	return 0;
}

#include <iostream>
#include <vector>
#include <ctime>
#include <algorithm>

using namespace std;

template <typename T>
void bubbleSort(vector<T> &vec) {
	bool swapped;

	for (int i=vec.size()-1; i>=0; i--) {
		swapped = false;
		for (int j=0; j<i; j++) {
			if (vec[j] > vec[j+1]) {
				swapped = true;
				swap(vec[j], vec[j+1]);
			}
		}

		if (!swapped) {
			break;
		}
	}
}

int main() {
	int num;

	cout << "Enter number: ";
	cin >> num;
	vector<int> vec(num, 0);

	for (int i=0; i<num; i++) {
		vec[i] = num-i;
	}

	auto start = clock();
	// sort(vec.begin(), vec.end());
	bubbleSort(vec);
	auto end = clock();

	cout << "Elapsed: " << end - start << endl;

	for (int i=0; i< vec.size(); i++) {
		cout << vec[i] << " ";
	}
	cout << endl;

	return 0;
}

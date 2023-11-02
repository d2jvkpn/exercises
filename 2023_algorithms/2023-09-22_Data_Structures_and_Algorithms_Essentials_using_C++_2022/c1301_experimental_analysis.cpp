# include <iostream>
# include <vector>
# include <ctime>
# include <algorithm>

using namespace std;

int main() {
	int num;

	cout << "Enter number: ";
	cin >> num;
	vector<int> vec(num, 0);

	for (int i=0; i<num; i++) {
		vec[i] = num-i;
	}

	auto start = clock();
	sort(vec.begin(), vec.end());
	auto end = clock();

	cout << "Elapsed: " << end - start << endl;

	return 0;
}

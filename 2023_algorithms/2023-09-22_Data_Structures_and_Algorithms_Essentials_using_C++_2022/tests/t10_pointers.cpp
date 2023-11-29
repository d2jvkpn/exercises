#include <iostream>
#include <vector>

using namespace std;

void alt(vector<int> &vec) {
	vec[0] = 42;
}

int main() {
	vector<int> vec = {1, 2, 3 };
	alt(vec);

	for (int i=0; i<vec.size(); i++) {
		cout << vec[i] << " ";
	}
	cout << endl;

	return 0;
}

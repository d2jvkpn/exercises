# include <iostream>
# include <vector>

using namespace std;

void push1(vector<int> vec) {
	vec.push_back(42);
}

void push2(vector<int>* vec) {
	vec->push_back(42);
}

void push3(vector<int>& vec) {
	vec.push_back(101);
}

int main() {
	vector<int> v1 = {1, 2, 3};

	push1(v1); // a clone of vec
	for (int i=0; i<v1.size(); i++) {
		cout << v1[i] << ", ";
	}
	cout << endl; // 1, 2, 3

	push2(&v1); // a pointer of vec
	for (int i=0; i<v1.size(); i++) {
		cout << v1[i] << ", ";
	}
	cout << endl; // 1, 2, 3, 42

	push3(v1);
	for (int i=0; i<v1.size(); i++) {
		cout << v1[i] << ", ";
	}
	cout << endl; // 1, 2, 3, 42, 101

	return 0;
}

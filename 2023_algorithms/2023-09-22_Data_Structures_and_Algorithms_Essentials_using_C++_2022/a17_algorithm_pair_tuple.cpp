#include <iostream>
#include <algorithm>
#include <vector>
#include <tuple>

using namespace std;

int calcTotalMarks(vector<int> marks) {
	return marks[0] + marks[1] + marks[2];
}

bool compare(pair<string, vector<int>> s1, pair<string, vector<int>> s2) {
	return calcTotalMarks(s1.second) > calcTotalMarks(s2.second);
}

int main() {
	// cout << "Hello, world!\n";

	//
	vector<int> vec = {10, 11, 2, 3, 4, 6, 7, 8};
	int key = 11;

	vector<int>::iterator it = find(vec.begin(), vec.end(), key);

	if (it == vec.end()) {
		cout << "Element not found" << endl;
	} else {
		cout << "Present at index: " << it - vec.begin() << endl;
	}

	//
	vector<pair<string, vector<int>>> students = {
		{"Rohan", {10, 20, 11}},
		{"Prateek", {10, 21, 3}},
		{"Vivek", {4, 5, 6}},
		{"Rijul", {10, 13, 20}},
	};


	sort(students.begin(), students.end(), compare);

	for (auto s: students) {
		cout << s.first << ": " << calcTotalMarks(s.second) << endl;
	}

	//
	tuple<int, double, string> myTuple(1, 3.14, "Hello");
	int x;
	double y;
	string z;
	tie(x, y, z) = myTuple;

	cout << x << ", " << y << ", " << z << endl;

	return 0;
}

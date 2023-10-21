# include <iostream>
# include <vector>

using namespace std;

int main() {
	// cout << "Hello, world!\n";
	vector<int> vec = {1, 2, 3, 4};

	cout << vec.size() << ", " << vec.capacity() << endl;	
	vec.push_back(42);	
	cout << vec.size() << ", " << vec.capacity() << endl;

	for (int i=0; i<vec.size(); i++) {
		cout << vec[i] << ", ";
	}
	cout << endl;

	vec.pop_back();
	cout << vec.size() << ", " << vec.capacity() << endl;

	//
	vector<int> visited(8, 1);
	for (int i=0; i<visited.size(); i++) {
		cout << "visited[" << i << "]=" << visited[i] << endl; 
	}

	//
	vector<vector<int>> matrix = {
		{1, 2, 3, 4},
		{5, 6, 7, 8},
		{9, 10, 11, 12},
		{13, 14, 15, 16},
	};

	for (int i=0; i<matrix.size(); i++) {
		for (int val: matrix[i]) {
			cout << val << ", ";
		}
		cout << endl;
	}

	return 0;
}

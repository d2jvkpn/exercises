#include <iostream>
#include <vector>

using namespace std;

int main() {
	vector<int> vec1 = {1, 2, 10, 12, 15};

	cout << "vec1 size: " << vec1.size() << ", capacity: " << vec1.capacity() << endl;
	// 5, 5

	vec1.push_back(16);
	cout << "vec1 size: " << vec1.size() << ", capacity: " << vec1.capacity() << endl;
	// 6, 10

	vec1.pop_back();
	cout << "vec1 size: " << vec1.size() << ", capacity: " << vec1.capacity() << endl;
	// 5, 10

	vector<int> vec2(10, 0); // {0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	cout << "vec2 size: " << vec2.size() << ", capacity: " << vec2.capacity() << endl;
	// 10, 10

	vector<int> vec3;
	vec3.reserve(10);
	cout << "vec3 size: " << vec3.size() << ", capacity: " << vec3.capacity() << endl;
	// 0, 10

	return 0;
}

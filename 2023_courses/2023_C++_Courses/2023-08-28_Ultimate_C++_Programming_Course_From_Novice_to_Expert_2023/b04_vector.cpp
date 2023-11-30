#include <iostream>
#include <vector>

#include "b02_helper.h"

using namespace std;

int main() {
    vector<int> vec;

	cout << vec.size() << endl;
	vec.reserve(3);
	vec.push_back(1);
	cout << vec[0] << ", " << vec.capacity() << ", " << vec.size() << endl;

	vec.push_back(2);
	vec.push_back(3);
	vec.push_back(4);

	cout << vec[0] << ", " << vec.capacity() << ", " << vec.size() << endl;
	
    return 0;
}


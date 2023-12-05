#include <iostream>
#include <algorithm>
#include <cctype>
#include <locale>
#include <sstream>

#include <vector>

using namespace std;

// trim from both ends (in place)
static inline int trim(string &s) {
	// static inline void ltrim(string &s)
	s.erase(s.begin(), find_if(s.begin(), s.end(), [](int ch) {
		return !isspace(ch);
	}));

	// static inline void rtrim(string &s)
	s.erase(find_if(s.rbegin(), s.rend(), [](int ch) {
		return !isspace(ch);
	}).base(), s.end());

	return s.length();
}

template <typename T>
void showArray(T* arr, int size) {
	// cout << "Array<" << typeid(T).name() << "> { ";
	cout << "Array { ";

	for (int i=0; i<size; i++) {
		if (i==size-1) {
			cout << arr[i];
		} else {
			cout << arr[i] << ", ";
		}
	}

	cout << " }\n";
}

template <typename T>
string arrayToString(T arr[], int size) {
	// cout << "Array<" << typeid(T).name() << "> { ";
	stringstream ss;

	ss << "Array { ";

	for (int i=0; i<size; i++) {
		ss << arr[i] << ", ";
	}

	string ans = ss.str();

	return ans.substr(0, ans.length()-2) + " }";
}

template <typename T>
void showVector(vector<T> &vec) {
	cout << "Vector { ";

	for (int i=0; i<vec.size(); i++) {
		if (i==vec.size()-1) {
			cout << vec[i];
		} else {
			cout << vec[i] << ", ";
		}
	}

	cout << " }\n";
}

template <typename T>
string vectorToString(vector<T> &vec) {
	// cout << "Array<" << typeid(T).name() << "> { ";
	stringstream ss;
	ss << "Vector { ";

	for (int i=0; i<vec.size(); i++) {
		if (i==vec.size()-1) {
			ss << vec[i];
		} else {
			ss << vec[i] << ", ";
		}
	}
	ss << " }";

	return ss.str();
}

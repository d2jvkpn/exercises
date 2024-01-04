#include <iostream>
#include <cassert>
#include <vector>

using namespace std;

class Vector {
	int* elems;
	int cap;
	int index;

public:
	Vector(int cap) {
		if (cap < 0) {
			length_error("constructor: negative capacity");
		}

		this->cap = cap;
		this->index = 0;
		this->elems = new int[cap];
	}

	int operator [](int i) const {
		if (i >= index) {
			throw out_of_range("index out of range");
		}

		return this->elems[i];
	}

	int size() const {
		return index;
	}

	int capacity() const {
		return cap;
	}

	void reserve(int cap) {
		if (cap <= this->cap) {
			return;
		}

		int* oldElems = elems;
		this->cap = cap;
		elems = new int[this->cap];
		cerr << "~~~ Expanding vector capacity to " << cap << endl;

		for (int i=0; i<index; i++) {
			elems[i] = oldElems[i];
		}

		delete [] oldElems;
	}

	void push(int val) {
		if (index >= cap) {
			reserve(2*cap);
		}

		elems[index] = val;
		index+=1;
	}

	int* last() const {
		if (index <= 0) {
			return NULL;
		}

		return &elems[index-1];
	}

	bool pop() {
		if (index <= 0) {
			return false;
		}

		int ans = elems[index-1];
		index-=1;

		return true;
	}

	~Vector() {
		delete [] this->elems;
	}
	
	// Vector(const Vector&); // copy constructor
	// Vector& operator=(const Vector&); // copy assignment

	// Vector(Vector&&); // move constructor
	// Vector& operator=(Vector&& a); // move assignment
};

ostream& operator<<(ostream& os, Vector& vec) {
	cout << "Vector: size=" << vec.size() << ", capacity=" << vec.capacity()  << ", elems={";

	for (int i=0; i<vec.size()-1; i++) {
		os << vec[i] << ", ";
	}

	if (vec.size() > 0) {
		os << vec[vec.size()-1] << "}";
	} else {
		os << "}";
	}

	return os;
}

Vector operator+(const Vector& a, const Vector& b) {
	int cap = a.size() > b.size() ? a.size() : b.size();
	int minSize = a.size() > b.size() ? b.size() : a.size();
	Vector ans(cap);

	for (int i=0; i<minSize; i++) {
		ans.push(a[i] + b[i]);
	}

	if (a.size() > b.size()) {
		for (int i=b.size(); i<a.size(); i++) {
			ans.push(a[i]);
		}
	} else {
		for (int i=a.size(); i<b.size(); i++) {
			ans.push(b[i]);
		}
	}

	return ans;
}

int main() {
	// cout << "Hello, world!\n";
	Vector vec1(3);
	cout << "==> vec1: " << vec1 << endl;
	assert(vec1.size()==0);
	assert(vec1.capacity()==3);

	//
	vec1.push(42);
	vec1.push(1);
	vec1.push(2);
	cout << "==> vec1: " << vec1 << endl;

	assert(vec1.size()==3);
	assert(vec1.capacity()==3);

	//
	vec1.push(3);
	cout << "==> vec1: " << vec1 << endl;

	assert(vec1.size()==4);
	assert(vec1.capacity()==6);

	//
	Vector vec2(1);
	vec2.push(8);
	Vector vec3 = vec1 + vec2;
	cout << "==> vec3: " << vec3 << endl;

	assert(vec3.size()==4);
	assert(vec3.capacity()==4);
	assert(vec3[0]==50);

	vector<int> v1(5);

	for (int i=0; i<v1.size(); i++) {
		cout << "v1["<< i << "]: " << v1[i] << endl;
	}

	for (auto p = v1.begin(); p != v1.end(); p++) {
		cout << "*p: " << *p << endl;
	}

	return 0;
}

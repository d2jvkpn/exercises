// assignment, copy, move, contructors, destructors
#include <iostream>
#include <vector>

using namespace std;

class X {
public:
	X() {}; // default constructor
	~X() {}; // destructor
	// X(const X&); // copy constructor
	X(X&&); // move constructor
	X& operator=(const X&); // copy assignment
	X& operator=(const X&&); // move assignment
};

class Y {
	Y();
	Y(const Y&) = default; // default copy constructor
	Y(Y&&) = default; // default move constructor
};

class Z {
public:
	vector<int> vec;
	string name;

	Z() {
		vec.reserve(7);
		name = "Z0";
	}

	Z& operator=(const Z&) = default; // default copy assignment

	void show() {
		cout << "~~~ Z: " << name << endl;
	}
};

class Shape {
public:
	string name = "Shape0";

	Shape() {};

	~Shape() {
		cerr << "!!! delete Shape: " << name << endl;
	};

	Shape& operator=(const Shape&) = delete; // delete can be used to suppress any function
	Shape operator=(const Shape) = delete;

	void show() {
		cout << "~~~ Shape: " << name << endl;
	}
};

/*
void copy(Shape& s1, Shape& s2) {
	s1 = s2; // error: Shape copy is deleted
}
*/

int main() {
	// cout << "Hello, world!\n";

	//
	X x1;

	//
	Z z1, z2;
	z1.name = "Z1";
	z2 = z1;

	z1.show(); // ~~~ Z: Z1
	z2.show(); // ~~~ Z: Z1

	//
	Shape s1;
	Shape s2;
	// s2 = s1; // error: use of deleted function ‘Shape& Shape::operator=(Shape&)’
	s2.name = "Shape2";

	s1.show();
	s2.show();

	return 0;
}

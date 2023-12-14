#include <iostream>

using namespace std;

class Container {
public:
	virtual int size() const = 0; // const member function, must defined
	virtual ~Container() {} // destructor
	virtual bool push(int) = 0; // push element, must defined
	virtual int& operator[](int i) = 0; //pure virtual funciton, must defined
	virtual void show() {
		cout << "~~ CONTAINER show" << endl;
	}
};

void use(Container& c){
	const int sz = c.size();

	for (int i=0; i<sz; ++i) {
		cout << c[i] << '\n';
	}
}

class Vector: public Container { // Vector implements Container
public:
	Vector(int cap) : elem{new int[cap]}, sz(0), cap(cap) { }
	// Vector of s elements

	bool push(int val) {
		if (sz >= cap) {
			return false;
		}
	
		sz+=1;
		elem[sz-1] = val;
	
		return true;
	}

	int& operator[](int i) { return elem[i]; }

	int size() const override {
		return sz;
	}

	// alternative implement
	void show() {
		cout << "~~ VECTOR show" << endl;
	}

private:
	int* elem;
	int sz, cap;
};

int main() {
	// cout << "Hello, world!\n";
	Vector vec(3);

	vec.push(1);
	vec.push(2);
	vec.push(42);
	vec.push(27);

	vec.show();
	use(vec);

	cout << vec.size() << endl;

	return 0;
}
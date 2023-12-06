#include <iostream>

using namespace std;

class Base {
public:
	virtual void show() {
		cout << "Base class\n";
	}
};

class Derived : public Base {
	public:
	void show() {
		cout<< "Derived class from Base\n";
	}
};

class Animal {
public:
	virtual ~Animal() {} // Good practice to have a virtual destructor

	virtual void eat() = 0; // Pure virtual function; it has to be overridden by any derived class

	virtual void sleep() {
		cout << "The animal sleeps" << endl;
	};
};

// This won't compile because Dog did not override Sleep()
class Dog: public Animal {
public:
	void eat() override {
		cout << "The dog eats." << endl;
	}
};

// This will compile and run
class Cat: public Animal {
public:
	void eat() override {
		cout << "The cat eats." << endl;
	}

	void sleep() override {
		cout << "The cat sleeps." << endl;
	}
};


int main() {
	// cout << "Hello, world!\n";
	Derived derived;
	derived.show();

	Dog dog;
	dog.eat();
	dog.sleep();

	Cat cat;
	cat.eat();
	cat.sleep();

	return 0;
}

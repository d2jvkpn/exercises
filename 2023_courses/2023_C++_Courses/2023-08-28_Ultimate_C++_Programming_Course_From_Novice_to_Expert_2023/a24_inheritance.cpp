#include <iostream>

using namespace std;

//
class Parent {
	public:
		int p;
};

class Child: public Parent {
	public:
		int c;

	Child(int _p, int _c) {
		p = _p;
		c = _c;
	}

	void display() {
		cout << "p: " << p << ", c: " << c << endl;
	};
};

//
class Vehicle {
	public:
		Vehicle() {
			cout << "I'm the constructor of Vehicle class." << endl;
		}

		void display() {
			cout << "Vehicle" << endl;
		}
};

class Car {
	public:
		Car() {
			cout << "I'm the constructor of Car class." << endl;
		}
};

class Sub: public Vehicle, public Car {
	public:
		void display() {
			cout << "Sub" << endl;
		}
};

int main() {
	Child child(4, 2);
	child.display();

	Sub sub;
	sub.display();

	return 0;
}

#include <iostream>
#include <string>

using namespace std;

class Employee {
	public:
		int id;
		string name;
		float sal;

		// Employee() { // constructed
		// 	cout << "==> constructed" << endl;
		// }

		Employee(int _id, string _name, float _sal) { // constructed
			cout << "==> constructed" << endl;

			id = _id;
			name = _name;
			sal = _sal;
		}

		void insert(int _id, string _name, float _sal) {
			id = _id;
			name = _name;
			sal = _sal;
		}

		void show() {
			cout << id << ", " << name << ", " << sal << "." << endl;
		}

	// private:
	// protected:
};


class Rectangle {
	public:
		int length;
		int breath;

		Rectangle(int _length, int _breath) {
			length = _length;
			breath = _breath;
		};

		int area() {
			return length*breath;
		};
};

int main() {
	// Employee e1;
	Employee e1(0, "", 0.0);
	e1.show();

	e1.insert(1, "Evol", 42.0);
	e1.show();

	e1.insert(4, "Adam", 24.0);
	e1.show();

	Rectangle r1(4, 4);
	cout << "Area: " << r1.area() << endl;

	return 0;
}

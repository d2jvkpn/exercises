#include <iostream>
#include <cstring>

using namespace std;

# define MAX 10

//
class Test {
	mutable int a;
	int b;

	public:
		Test(int _a, int _b) {
			a = _a;
			b = _b;
		}

		void square_a() const {
			a = a*a;
		}

		void display() const {
			cout << "a = " << a << ", b = " << b << endl;
		}
};

//
class Student {
	private:
		string name;
		int rollNo;
		int total;
		float perc;

	public:
		void getDetails();
		void putDetails();
};

void Student::getDetails() {
	cout << "Enter name: ";
	cin >> name;

	cout << "Enter roll number: ";
	cin >> rollNo;

	cout << "Enter total marks out of 500: ";
	cin >> total;

	perc = (float)total/500*100;
};

void Student::putDetails() {
	cout << "Student Details: " << endl;
	cout << "  name: " << name << endl;
	cout << "  roll no: " << rollNo << endl;
	cout << "  total: " << total << endl;
	cout << "  percentage: " << perc << endl;
};

//
class Number {
	private:
		int a;

	public:
		void getNum(int _a);
		friend void printNum(Number number);
};

void Number::getNum(int _a) {
	a = _a;
};

void printNum(Number number) {
	cout << "Value of a (private data member of class Number): " << number.a << endl;
};

int main() {
	//
	cout << "==>" << endl;
	const Test test(2, 3);
	test.display();

	test.square_a();
	test.display();

	//
	cout << "==>" << endl;
	Student student;
	student.getDetails();
	student.putDetails();

	//
	cout << "==>" << endl;
	Number number;
	number.getNum(42);
	printNum(number);

	return 0;
}

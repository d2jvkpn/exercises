# include <iostream>
# include <vector>

using namespace std;

class Person {
	private:
		double height;

	public:
		string name;
		int age;
		string gender;

		Person(string _name, int _age, string _gender) {
			cout << "==> Constructing Object " << _name << "!\n";
			name = _name;
			age = _age;
			gender = _gender;
			height = 0;
		}

		Person() {
			cout << "==> Constructing without arguments!\n";
			name = "unknown";
			age = 0;
			gender = "unknown";
			height = 0;
		}

		void setHeight(double _height) {
			height = _height;
		}

		double getHeight() {
			return height;
		}

		void printInfo() {
			cout << name << " is " << age << " years old.\n";
		}

		void add() {
			age += 1;
		}
};

class Employee : public Person {
	private:	
		double salary;

	public:
		Employee(string _name, int _age, string _gender, double _salary) : Person (_name, _age, _gender) {
			salary = _salary;
		}

		double getSalary() {
			return salary;
		}
};

int main() {
	// Person ivan("Ivan", 21, "female");
	Person ivan = Person("Ivan", 21, "female");
	ivan.add();
	ivan.printInfo();

	ivan.setHeight(1.72);
	cout << "The height of " << ivan.name << " is " << ivan.getHeight() << ".\n";

	Person unknown = Person();
	unknown.printInfo();

	Employee julia = Employee("Julia", 21, "female", 102.4);
	julia.printInfo();
	cout << "The salary of " << julia.name << " is " << julia.getSalary() << "k.\n";

	return 0;
}

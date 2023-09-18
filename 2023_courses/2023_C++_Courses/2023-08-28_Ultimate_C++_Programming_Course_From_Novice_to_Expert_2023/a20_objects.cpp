# include <iostream>
# include <string>

using namespace std;

class Person {
	public:
		int yearOfBirth;
		string name;
};

class Math{
	public:
		double length;
		double breath;
		double height;

		double volume() {
			return area()*height;
		}

	private:
		double area() {
			return length*breath;
		}

};

int main() {
	Person evol;

	evol.yearOfBirth = 2001;
	evol.name = "Evol";
	cout << evol.name << ", " << evol.yearOfBirth << "." << endl;

	Math obj;
	obj.length = 5.0;
	obj.breath = 7.0;
	obj.height = 10.0;

	cout << "volume: " << obj.volume() << "." << endl;

	return 0;
}

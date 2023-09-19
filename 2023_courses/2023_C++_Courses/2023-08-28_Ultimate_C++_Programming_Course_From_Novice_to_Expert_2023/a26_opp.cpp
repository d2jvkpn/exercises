# include <iostream>
# include <cstring>

using namespace std;

//
class Shape {
	protected:
		float height, width;

	public:
		Shape() {
			height = 0.0;
			width = 0.0;
		}

		void set(float _height, float _width) {
			height = _height;
			width = _width;
		}

		void display() {
			cout << "height: " << height << ", width: " << width << endl;
		}
};

class Rectangle: public Shape {
	public:
		float area() {
			return height * width;
		}
};

class Triangle: public Shape {
	public:
		float area() {
			return height * width / 2.0;
		}
};

//
class Animal {
	protected:
		int age;
		string name;

	public:
		void set(int _age, string _name) {
			age = _age;
			// strcpy(_name, name); // char name[20];
			name = _name;
		}

		virtual void msg() = 0;
};

class Zebra: public Animal {
	public:
		void msg() {
			cout << "The zebra named " << name << " is " << age << " year(s) old." << endl;
		}
};

class Dolphin: public Animal {
	public:
		void msg() {
			cout << "The dolphin named " << name << " is " << age << " year(s) old." << endl;
		}
};

int main() {
	//
	Rectangle r;
	Triangle t;

	r.display();
	t.display();

	r.set(1.2, 2.0);
	t.set(4.2, 2.0);

	r.display();
	cout << "Rectangle area: " << r.area() << endl;

	t.display();
	cout << "Triangle area: " << t.area() << endl;

	//
	Zebra zebra;
	Dolphin dolphin;

	char name1[20] = "abc";
	char name2[20] = "xyz";

	zebra.set(8, name1);
	dolphin.set(2, name2);

	zebra.msg();
	dolphin.msg();

	return 0;
}

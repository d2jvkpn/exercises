#include <iostream>
#include <string>

using namespace std;


class Shape {
	public:
		virtual float getArea() = 0; // must be implement by derived class

		virtual void msg() { // a derived class can implement it's msg method
			cout << "Msg from Shape class." << endl;
		}

		void setWidth(float _width) {
			width = _width;
		}

		void setHeight(float _height) {
			height = _height;
		}

	protected:
		float width;
		float height;
};

class Rectangle: public Shape {
	public:
		Rectangle(float _width, float _height) {
			width = _width;
			height = _height;
		}

		float getArea() {
			return width * height;
		}
};

class Triangle: public Shape {
	public:
		Triangle(float _width, float _height) {
			width = _width;
			height = _height;
		}

		void msg() {
			cout << "Msg from Triangle class." << endl;
		}

		float getArea() {
			return width * height / 2.0;
		}
};

int main() {
	Rectangle r(3.0, 6.0);
	Triangle t(4.0, 2.0);

	cout << "==> Rectangle: " << r.getArea() << endl;
	cout << r.getArea() << endl;
	r.setWidth(4.0);
	cout << r.getArea() << endl;
	r.msg();

	cout << "==> Trinagle: " << t.getArea() << endl;
	cout << t.getArea() << endl;
	t.setWidth(3.0);
	cout << t.getArea() << endl;
	t.msg();

	return 0;
}

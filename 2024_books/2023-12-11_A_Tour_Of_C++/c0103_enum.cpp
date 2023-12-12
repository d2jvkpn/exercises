#include <iostream>

using namespace std;

enum class Color { red, blue, green };
enum class TrafficLight { green, yellow, red };


ostream& operator<<(ostream &os, const Color& c) { 
	switch (c) {
	case Color::red:
		os << "RED";
		break;
	case Color::blue:
		os << "BLUE";
		break;
	case Color::green:
		os << "GREEN";
		break;
	}

	return os;
}

// https://en.cppreference.com/w/cpp/language/operator_incdec

// prefix increment: ++
TrafficLight& operator++(TrafficLight& t){
	switch (t) {
	case TrafficLight::green:
		t = TrafficLight::yellow;
		break;
	case TrafficLight::yellow:
		t = TrafficLight::red;
		break;
	case TrafficLight::red:
		t = TrafficLight::green;
		break;
	}

	return t;
}

// post increment: ++
void operator++(TrafficLight& t, int){
	switch (t) {
	case TrafficLight::green:
		t = TrafficLight::yellow;
		break;
	case TrafficLight::yellow:
		t = TrafficLight::red;
		break;
	case TrafficLight::red:
		t = TrafficLight::green;
		break;
	}
}

ostream& operator<<(ostream &os, const TrafficLight& t) { 
	switch (t) {
	case TrafficLight::green:
		os << "Green";
		break;
	case TrafficLight::yellow:
		os << "Yellow";
		break;
	case TrafficLight::red:
		os << "Red";
		break;
	}

	return os;
}

int main() {
	// cout << "Hello, world!\n";

	Color color = Color::red;
	auto light = TrafficLight::red;

	int x = int (Color::blue);
	cout << x << endl; // 1

	cout << "==> Light:" << endl;
	cout << light << endl;
	++light;
	cout << light << endl;
	light++;
	cout << light << endl;
	light++;
	cout << int(light) << endl;

	return 0;
}
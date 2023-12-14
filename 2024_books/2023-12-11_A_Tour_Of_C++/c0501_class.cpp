#include <iostream>
#include <cmath>

using namespace std;

class Complex {
	double re, im;

public:
	Complex(double re, double im): re{re}, im{im} {}
	Complex(double re): re{re}, im{0} {}
	Complex(): re{0}, im{0} {}

	double real() const {
		return re;
	}
	void real(double re) {
		this->re = re;
	}

	double imag() const {
		return im;
	}
	void imag(double im) {
		this->re = im;
	}

	Complex& operator+=(Complex z) {
		re+=z.re;
		im+=z.im;
		return *this;
	}

	Complex& operator-=(Complex z) {
		re-=z.re;
		im-=z.im;
		return *this;
	}

	Complex& operator*=(Complex&); // defined out-of-class somewhere
	void operator/=(Complex&); // defined out-of-class somewhere
};

// cout << complex << endl;
ostream& operator<<(ostream& os, Complex& z) {
	if (z.imag() >= 0) {
		os << z.real() << "+" << z.imag() << "i";
	} else {
		os << z.real() << z.imag() << "i";
	}

	return os;
}

// c=a+b
Complex operator+(Complex a, Complex b) {
	return {a.real() + b.real(), a.imag() + b.imag()};
}

// c=a-b
Complex operator-(Complex a, Complex b) {
	return {a.real() - b.real(), a.imag() - b.imag()};
}

// -a
Complex operator-(Complex a) {
	return {-a.real(), -a.imag()};
}

// a == b
bool operator==(Complex a, Complex b) {
	return a.real() == b.real() && a.imag() == b.imag();
}

// a != b
bool operator!=(Complex a, Complex b) {
	return a.real() != b.real() || a.imag() != b.imag();
}

// a*=b
Complex& Complex::operator*=(Complex& b) {
	double r = this->real() * b.real() - this->imag() * b.imag();
	double i = this->real() * b.imag() + this->imag() * b.real();
	this->real(r);
	this->imag(i);

	return *this;
}

// a/=b
void Complex::operator/=(Complex& b) {
	double d = sqrt(this->real()) + sqrt(b.imag()); // denominator
	double r = this->real() * b.real() + this->imag() * b.imag();
	double i = this->imag()* b.real() - this->real() * b.imag();

	this->real(r/d);
	this->imag(i/d);
}

int main() {
	// cout << "Hello, world!\n";
	Complex z1 = {1, 0};
	// const Complex z2 {1, 3};

	// z1 = z2; // ok
	// z2 = z1; // error: assignment to const

	Complex z2 {1, 3};
	double x = z1.real();

	cout << "z1: " << z1 << ", " << z2 << endl;

	z1 += z2;
	cout << "z1: " << z1 << endl;

	z1 /= z2;
	cout << "z1: " << z1 << endl;

	Complex z3 = {3, -4};
	Complex z4 = {5, -6};

	Complex z5 = z3 + z4;
	// cout << (z3 + z4) << endl; // can't compile
	cout << "z5: " << z5 << endl;

	Complex z6 = -z3;
	cout << z6 << endl;

	return 0;
}
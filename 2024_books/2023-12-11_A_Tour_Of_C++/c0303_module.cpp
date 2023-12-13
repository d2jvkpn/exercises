// $ g++ -g -fmodules-ts -std=gnu++20 -o ./target/c0303 c0303.cpp include/vector.cpp
import Vector;
#include <iostream>

using namespace std;

double sqrt(double val) {
    return val*val;
}

// why can't put this function in include/vector.cpp
ostream& operator<<(ostream& os, Vector& vec) {
	os << "Vector {";

	for (int i=0; i<vec.size(); i++) {
		os << vec[i] << ", ";
	}

	os << "\b\b}";

    return os;
}

int main() {
    Vector vec(7);

    cout << vec << endl;

    return 0;
}

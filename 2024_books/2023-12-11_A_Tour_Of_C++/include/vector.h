#include <iostream>

using namespace std;

class Vector {
public:
    Vector(int s) {
        this->elem = new int[s];
        sz = s;

        for (int i=0; i<s; i++) {
            this->elem[i] = i;
        }
    }

    int& operator[](int i) {
        return elem[i];
    }

    int size() {
        return sz;
    }

private:
    int* elem;
    int sz;
};

ostream& operator<<(ostream& os, Vector& vec) {
	os << "Vector {";

	for (int i=0; i<vec.size(); i++) {
		os << vec[i] << ", ";
	}

	os << "\b\b}";


    return os;
}

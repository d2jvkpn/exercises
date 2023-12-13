#include <iostream>

using namespace std;

double sqrt(double);

class Vector {
public:
    Vector(int s);
    double& operator[](int i);
    int size();

private:
    double* elem;
    int sz;
};

int main() {
    cout << sqrt(4.2) << endl;

    Vector vec(7);

    cout << vec.size() << endl;

    return 0;
}

double sqrt(double val) {
    return val*val;
}

Vector::Vector(int s) {
    double* elem = new double[s];
    sz = s;
};

double& Vector::operator[](int i) {
    return elem[i];
}

int Vector::size() {
    return sz;
}
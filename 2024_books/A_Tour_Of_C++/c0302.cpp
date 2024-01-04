#include <iostream>

#include "include/vector.h"

using namespace std;

double sqrt(double val) {
    return val*val;
}

int main() {
    Vector vec(7);

    cout << vec << endl;

    return 0;
}


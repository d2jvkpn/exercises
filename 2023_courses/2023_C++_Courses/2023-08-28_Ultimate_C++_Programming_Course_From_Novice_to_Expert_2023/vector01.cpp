#include <iostream>
#include <vector>

# include "helper.h"

using namespace std;

int main() {
    vector<int> vec = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    vector<int> v2 = vector_clone(vec, 2, 6); // a new vector

    printVector(vec);
    printVector(v2);

	v2[0] = 42;

    printVector(vec);
    printVector(v2);

    return 0;
}


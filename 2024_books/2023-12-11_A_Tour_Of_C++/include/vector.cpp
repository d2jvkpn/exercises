module;

#include <string_view>

export module Vector;

using namespace std;

export class Vector {
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

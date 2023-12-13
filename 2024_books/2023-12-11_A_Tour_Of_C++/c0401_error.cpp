#include <iostream>

using namespace std;

class Vector {
public:
	Vector(int s) {
		if (s < 0) {
			throw length_error("Vector constructor: negative size");
		}

		elem = new int[s];
		
		sz = s;
	}

private:
	int* elem;
	int  sz;
};

void test(int n) {
	try {
		Vector vec(n);
	} catch (length_error& err) {
		cerr << "... length_error" << endl;
	} catch (bad_alloc& err) {
		cerr << "... bad_alloc" << endl;
	}
}

int main() {
	// cout << "Hello, world!\n";

	test(-27);
	test(1'000'000'000);
	test(42);

	return 0;
}

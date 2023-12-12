#include <iostream>

using namespace std;

class Vector {
public:
	Vector(int sz): elem{new double[sz]}, sz{sz} {}

	int size() {
		return sz;
	}

	double& operator[](int i) {
		if (i >= size()) {
			cerr << "!!! out of index\n";
		}
		return elem[i];
	}

private:
	double* elem;
	int sz;
};

double read_and_sum(int s) {
	Vector vec(s);

	printf("==> Enter %d numbers:\n", s);
	for (int i=0; i<vec.size(); i++) {
		cin >> vec[i];
	}

	double ans = 0;
	for (int i=0; i<vec.size(); i++) {
		ans += vec[i];
	}

	return ans;
}

int main() {
	// cout << "Hello, world!\n";
	Vector vec(6);

	cout << vec[12] << endl;

	// double ans = read_and_sum(3);
	// cout << "ans: " << ans << endl;

	vec[3] = 42;
	cout << vec[3] << endl;

	int value = vec[3];
	value = 33;
	cout << vec[3] << endl;

	return 0;
}
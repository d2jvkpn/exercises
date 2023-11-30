#include <iostream>

using namespace std;

template <class T>
T multiply(T a, T b) {
	return a*b;
}

template <class T>
class MathTools {
	public:
		T add(T a, T b) {
			return a + b;
		}


		T subtract(T a, T b) {
			return a - b;
		}

		T multiply(T a, T b) {
			return a + b;
		}

		T divide(T a, T b) {
			return a / b;
		}
};

typedef MathTools<double> DoubleTools;

int main() {
	cout << "1 * 2 = " << multiply(1, 2) << endl;
	cout << "1.2 * 2.1 = " << multiply(1.2, 2.1) << endl;
	// cout << "1.2 * 2 = " << multiply(1.2, 2) << endl; // can't compile

	MathTools<int> mtInt;
	cout << mtInt.add(1, 2) << endl;

	DoubleTools mtDouble;
	cout << mtDouble.add(1.2, 2.1) << endl;
	cout << mtDouble.add(1.2, 2) << endl;

	return 0;
}

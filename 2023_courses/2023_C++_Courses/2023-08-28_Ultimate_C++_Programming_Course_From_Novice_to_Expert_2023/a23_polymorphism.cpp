#include <iostream>

using namespace std;

class Count {
	private:
		int value;

	public:
		Count() {
			value = 8;
		}

		// https://en.cppreference.com/w/cpp/language/operator_incdec
		// ++Count
		void operator ++ () {
			value += 1;
		}

		// Count++
		void operator ++ (int) {
			value+=1;
		}

		void display() {
			cout << "Count.value: " << value << endl;
		}
};

int sum(int num1, int num2) {
	return num1 + num2;
}

double sum(double num1, double num2) {
	return num1 + num2;
}

int main() {
	Count count;
	count.display();

	count++;
	count.display();

	int num1 = 4, num2 = 2;
	cout << "4 + 2 = " << sum(num1, num2) << endl;
	cout << "4.0 + 2.0 = " << sum(4.0, 2.0) << endl;

	return 0;
}

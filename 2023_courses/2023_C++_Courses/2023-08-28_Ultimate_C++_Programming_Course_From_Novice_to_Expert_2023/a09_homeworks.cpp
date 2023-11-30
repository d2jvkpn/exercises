#include <iostream>
#include <cmath>

using namespace std;

bool isPrime(int num) {
	for (int i = 2; i <= sqrt(num); i++) {
		if (num%i == 0) {
			return false;
		}
	}

	return true;
}

int convertBinaryToDecimal(long long num) {
	int deci = 0, i = 0, remaider;

	while (num!=0) {
		remaider = num % 10;
		num /= 10;
		deci += remaider * pow(2, i);
		i+=1;
	}

	return deci;
};

int main() {
	cout << isPrime(1024) << endl;

	long long n;
	cout << "Enter a Binary number: ";
	cin >> n;

	cout << n << " in binary = " << convertBinaryToDecimal(n) << " in decimal" << endl;

	return 0;
}

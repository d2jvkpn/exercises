# include <iostream>

using namespace std;

void clearBitsRange(int &n, int i, int j) {
	int a = (~0) << (j+1);
	int b = (1<<i) - 1;
	int mask = a | b;

	n = n & mask;
}

bool isPowerOf2(int n) {
	return (n & (n-1)) == 0;
}

int coutBits(int n) {
	int count = 0;

	while (n>0) {
		count += (n & 1);
		n = n >> 1;
	}

	return count;
}

int coutBitsHack(int n) {
	int ans = 0;

	while (n>0) {
		n = n & (n-1);
		ans += 1;
	}

	return ans;
}

int fastExpo(int a, int n) {
	int ans = 1;

	while (n > 0) {
		if (n & 1) {
			ans *= a;
		}

		a *= a;
		n = n >> 1;
	}

	return ans;
}

int convert2Binary(int n) {
	int ans = 0, p = 1;

	while (n > 0) {
		ans += p*(n&1);
		p*=10;
		n = n >> 1;
	}

	return ans;
}

int main() {
	// cout << "Hello, world!" << endl;

	// operations: ^, |, ~, <<, >>
	cout << "(5 | 8) = " << (5 | 8) << endl;
	cout << "(5 & 8) = " << (5 & 8) << endl;
	cout << "5 << 2 = " << (5 << 2) << endl;
	cout << "20 >> 2 = " << (20 >> 2) << endl;

	// odd and even
	int x = 20;
	if (x & 1) {
		cout << x << " is odd" << endl;
	} else {
		cout << x << " is even" << endl;
	}

	// get ith bit
	int i = 1, mask = (1<<i);
	// cout << "5 get " << i << "th bit: " << ((5 & mask) > 0 ? 1 : 0) << endl;
	cout << "5 get " << i << "th bit: " << ((5 & mask) >> i) << endl;	
	cout << "5 get " << i << "th bit: " << ((5 >> i) & 1) << endl;

	// set ith bit
	cout << "5 set " << i << "th bit: " << (5 | mask) << endl;

	// clear ith bit
	cout << "5 clear " << i << "th bit: " << (5 & (~mask)) << endl;

	//
	int n = 31;
	clearBitsRange(n, 1, 3);
	cout << "(1<<3)-1: " << ((1<<3) - 1) << endl;
	cout << "n: " << n << endl;

	// cout bits
	int val1;
	cout << "==> coutBits: ";
	cin >> val1;
	cout << "ans: " << coutBits(val1) << endl;

	// fast expo
	int val2, n2;
	cout << "==> fastExpo: ";
	cin >> val2 >> n2;
	cout << "ans: " << fastExpo(val2, n2) << endl;

	// convert to binary
	int val3;
	cout << "==> convert2Binary: ";
	cin >> val3;
	cout << "ans: " << convert2Binary(val3) << endl;

	return 0;
}

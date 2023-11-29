#include <iostream>

using namespace std;

int fact(int num) {
	if (num==0) {
		return 1;
	}

	return num*fact(num-1);
}

int fib(int num) {
	if (num==0 or num==1) {
		return num;
	}

	return fib(num-1) + fib(num-2);
}

int isSorted(int array[], int size) {
	if (size == 0 || size == 1) {
		return true;
	}

	return (array[0] < array[1]) && isSorted(array, size-1);
}

void dec(int num) {
	if (num == 0) {
		return;
	}

	dec(num - 1);
	if (num == 1) {
		cout << num;
	} else {
		cout << ", " << num;
	}
}

void inc(int num) {
	if (num == 0) {
		return;
	}

	if (num == 1) {
		cout << num;
	} else {
		cout << num << ", ";
	}

	inc(num - 1);
}

template<typename T>
int firstOcc(T array[], int size, T target) {
	if (size == 0) {
		return -1;
	}

	if (array[0] == target) {
		return 0;
	}

	int ans = firstOcc(array+1, size-1, target);

	if (ans == -1) {
		return -1;
	} else {
		return ans + 1;
	}
}

template<typename T>
int lastOcc(T array[], int size, T target) {
	if (size == 0) {
		return -1;
	}

	if (array[size-1] == target) {
		return size-1;
	}

	return lastOcc(array, size-1, target);
}

int power(int base, int e) {
	if (e==0) {
		return 1;
	}

	return base * power(base, e-1);
}

int fastPower(int base, int e) {
	if (e == 0) {
		return 1;
	}

	int subProb = fastPower(base, e/2);
	int subProbSqrt = subProb*subProb;

	if (e & 1) {
		return base*subProbSqrt;
	} else {
		return subProbSqrt;
	}
}

void bubbleSort(int array[], int size) {
	for (int i=1; i < size; i++) {
		for (int j=0; j<=size-i-1; j++) {
			if (array[j] > array[j+1]) {
				swap(array[j], array[j+1]);
			}
		}
	}
}

void bubbleSortRec(int array[], int size) {
	if (size == 1) {
		return;
	}

	for (int i=0; i < size-1; i++) {
		if (array[i] > array[i+1]) {
			swap(array[i], array[i+1]);
		}
	}

	bubbleSortRec(array, size-1);
}

string spell[] = {
	"zero", "one", "two", "three", "four",
	"five", "six", "seven", "eight", "nine",
};

void printSpell(int num) {
	if (num == 0) {
		cout << endl;
		return;
	}

	printSpell(num/10);
	cout << spell[num%10] << " ";
}

int main() {
	int num;
	cout << "==> Factorial: ";
	cin >> num;

	if (num >= 0) {
		cout << "ans: " << fact(num) << endl;
	} else {
		cerr << "invalid number" << endl;
		exit(1);
	}

	cout << "==> Fibonacci: ";
	cin >> num;
	if (num >= 0) {
		cout << "ans: " << fib(num) << endl;
	} else {
		cerr << "invalid number" << endl;
		exit(1);
	}

	cout << "==> Dec: ";
	cin >> num;
	dec(num);
	cout << endl;

	inc(num);
	cout << endl;

	int array[] = {1, 2, 7, 6, 7, 5};
	int size = sizeof(array)/sizeof(int);

	cout << firstOcc(array, size, 7) << endl;
	cout << lastOcc(array, size, 7) << endl;

	printSpell(2023);
	cout << endl;

	return 0;
}

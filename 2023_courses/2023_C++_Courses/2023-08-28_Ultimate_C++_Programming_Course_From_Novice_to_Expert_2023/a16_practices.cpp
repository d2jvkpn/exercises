# include <iostream>

using namespace std;

void fizzBuzz(int num) {
	for (int i = 1; i<=num; i++) {
		if (i%15 == 0) {
			cout << "FizzBuzz, ";
		} else if (i%3 == 0){
			cout << "Fizz, ";
		} else if (i%5 == 0) {
			cout << "Buzz, ";
		} else {
			cout << i << ", ";
		}
	}
	cout << endl;
}

void pyramid(int num) {
	int i, j;
	for(i=1; i<=num; i++) {
		for (j=1; j<=i; j++) {
			cout << "*";
		}
		cout << endl;
	}
}



int main() {
	fizzBuzz(100);
	pyramid(6);
 
	return 0;
}


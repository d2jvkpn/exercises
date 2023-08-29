# include <iostream>

using namespace std;

int globalNumber = 10; 

void test() {
	cout << "Testing...\n";
}

int multiply(int a, int b) {
	return a * b;
}

string createGreeting(string name) {
	return "Greetings " + name + "!";
}

void printGlobalName() {
	cout << "globalNumber: " << globalNumber << endl;
}

int main() {
	test();
	test();
	test();

	int answer = multiply(6, 7);
	cout << answer << endl;

	cout << createGreeting("Evol") << endl;

	printGlobalName();

	int b = 10;
	if (b < 20) {
		int hello = 32;
		cout << hello << endl;
	}

	return 0;
}

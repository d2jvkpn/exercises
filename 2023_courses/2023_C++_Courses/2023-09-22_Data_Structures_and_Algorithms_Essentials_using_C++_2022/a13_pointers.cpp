# include <iostream>

using namespace std;

void applyTax(float &income) {
	float tax = 0.10;
	income = income - income*tax;
}

void applyTax2(float* income) {
	float tax = 0.10;
	*income = *income - *income*tax;
}

void applyTax3(float* income) {
	float tax = 0.10;
	float money = *income;

	money = money - money*tax;
	*income = money;
}


void viewVideo(int * cout) {
	*cout += 1;
}

int main() {
	// cout << "Hello, world!\n";

	//
	int x = 10;
	float y = 4.2;

	int* xp; // int *xp; // int * xp;
	xp = &x;

	cout << "==> x: " << &x << ", " << *(&x) << endl;
	cout << "==> xp: " << xp << ", " << &xp << ", " << *xp << endl;
	cout << "==> y: " << &y << endl;

	int** xpp = &xp;
	cout << "==> xpp: " << xpp << ", " << *xpp << ", " << **xpp << endl;

	//
	float income = 100;

	cout << "==> income: " << income;
	applyTax(income);
	cout << ", " << income << endl;

	applyTax2(&income);
	cout << "    " << income << endl;


	//
	int count = 100;
	viewVideo(&count);

	cout << "==> cout: " << count << endl;

	return 0;
}

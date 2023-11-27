// https://www.geeksforgeeks.org/passing-a-function-as-a-parameter-in-cpp/
# include <iostream>

using namespace std;

// Function that add two numbers
int add(int x, int y) { return x + y; }

// Function that multiplies two numbers
int multiply(int x, int y) { return x * y; }

int invoke(int x, int y, int (*func)(int, int))
{
    return func(x, y);
}

int main() {
	// cout << "Hello, world!\n";

    // Pass pointers to add & multiply
    // function as required
    cout << "Addition of 20 and 10 is ";
    cout << invoke(20, 10, &add) << '\n';
 
    cout << "Multiplication of 20"
         << " and 10 is ";
    cout << invoke(20, 10, &multiply) << '\n';

	return 0;
}

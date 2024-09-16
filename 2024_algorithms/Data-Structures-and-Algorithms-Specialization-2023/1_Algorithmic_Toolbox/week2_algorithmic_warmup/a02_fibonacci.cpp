#include <iostream>

long long fibonacci(long long n) {
    if (n <= 1) { return n; }

    long long previous = 0;
    long long current  = 1;

    for (long long i = 0; i < n - 1; ++i) {
        long long temp = previous;
        previous = current;
        current = temp + current;
    }

    return current;
}

int main() {
    long long num;

    std::cout << "Enter a number: ";
    std::cin >> num;

    std::cout << fibonacci(num) << std::endl;
}

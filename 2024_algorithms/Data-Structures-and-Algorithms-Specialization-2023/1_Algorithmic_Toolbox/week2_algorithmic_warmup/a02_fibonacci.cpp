#include <iostream>

// long long(2^63-1): 9,223,372,036,854,775,807
long long fibonacci(long long n) {
    if (n < 2) {
        return n;
    }

    long long v1 = 0, v2 = 1;

    for (long long i = 1; i < n; i++) {
        long long temp = v1;
        v1 = v2;
        v2 = temp + v1;
    }

    return v2;
}

int main() {
    long long num;

    std::cout << "Enter a number: ";
    std::cin >> num;

    std::cout << fibonacci(num) << std::endl;
}

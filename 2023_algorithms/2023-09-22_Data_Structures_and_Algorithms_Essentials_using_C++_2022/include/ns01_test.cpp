#include <iostream>
#include "ns01.h"

using namespace std;
using namespace ns01;

int main() {
	// cout << "Hello, world!\n";

	Stack<int> stack;

	stack.push(1);
	stack.push(2);
	stack.push(3);

	while (!stack.empty()) {
		cout << stack.top() << endl;
		stack.pop();
	}

	return 0;
}

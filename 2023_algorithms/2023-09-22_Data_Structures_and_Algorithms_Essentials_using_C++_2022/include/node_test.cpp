#include <iostream>
#include <array>
#include "node.h"

using namespace std;

template <typename T, size_t N>
using Array = array<T, N>;

int main() {
	cout << "Hello, world!\n";

	Node node(2);
	node.left = new Node(1);
	node.right = new Node(3);

	Array<int, 2> ans = node.height2();
	cout << ans[0] << ", " << ans[1] << endl;

	return 0;
}

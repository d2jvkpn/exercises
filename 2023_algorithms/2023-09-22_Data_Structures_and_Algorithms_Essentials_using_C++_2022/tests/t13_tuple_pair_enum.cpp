#include <iostream>
#include <tuple>

using namespace std;

template <typename... Types>
using Tuple = tuple<Types...>;

template <typename A, typename B>
using Pair = pair<A, B>;

enum class Side {
	Left,
	Right = 4,
};

int main() {
	cout << "Hello, world!\n";

	// Tuple	
	Tuple<int, string, double> tuple1 = make_tuple(42, "Hello", 2.7);

	cout << get<0>(tuple1) << ", " << get<1>(tuple1) << ", " << get<2>(tuple1) << endl;
	auto [a, b, c] = tuple1;
	cout << a << endl;

	// Pair
	Pair<int, string> pair1 = {42, "world"};
	cout << pair1.first << ", " << pair1.second << endl;
	auto [d, e] = pair1;
	cout << d << endl;

	// Enum
	Side left = Side::Left;
	Side right = Side::Right;
	cout << (left == right) << endl;

	return 0;
}

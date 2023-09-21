# include <iostream>

using namespace std;

namespace ns01 {
	int a = 42;

	void test() {
		cout << "I'm from namespace ns01" << endl;
	}

	namespace inner {
		const int b = a;
	}
};

namespace ns02 {
	void test() {
		cout << "I'm from namespace ns02" << endl;
	}
};

using namespace ns02;

int main() {
	ns01::test();
	test();

	cout << ns01::a << ", " << ns01::inner::b << endl;
	ns01::a = 1024;
	cout << ns01::a << ", " << ns01::inner::b << endl;

	return 0;
}

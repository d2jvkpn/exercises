module;

#include <iostream>
#include <string_view>

export module Hello;

using namespace std;

namespace ns01 {
	export void greeter(string_view const &name) {
		cout << "Hello " << name << "!\n";
	}
}

export class Data {
public:
	Data(int x): xvalue(x) {}

	void show() {
		cout << "Data { xvalue: " << xvalue << " }\n";
	}

private:
	int xvalue;
};

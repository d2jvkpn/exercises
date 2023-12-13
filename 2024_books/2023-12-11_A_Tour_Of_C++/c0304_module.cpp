// $ g++ -g -fmodules-ts -std=gnu++20 -o ./target/c0304_module c0304_module.cpp include/hello.cpp

import Hello;

int main (void) {
	ns01::greeter("world");

	Data data(42);
	data.show();

	return 0;
}

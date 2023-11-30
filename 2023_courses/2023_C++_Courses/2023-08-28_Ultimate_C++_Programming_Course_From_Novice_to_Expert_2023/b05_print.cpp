#include <iostream>

using namespace std;

int main() {
	char hello[] = "Hello, world!";

	printf("%s %d\n", hello, 42);

	cerr << hello << endl;
	fprintf(stderr, "stderr: %s\n", hello);
	fprintf(stdout, "stdout: %s\n", hello);

	return 0;
}

# include <iostream>
# include <unistd.h>
# include <csignal>

using namespace std;

# define PI 3.1415968

template <typename T>
T myMax(T x, T y) {
	return (x > y) ? x : y;
}

void signalHandler(int sig) {
	cout << "Interrupt signal (" << sig << ") recieved." << endl;
	exit(sig);
}

int main() {
	signal(SIGINT, signalHandler);

	cout << myMax<int>(2, 4) << endl;
	cout << myMax<float>(2.4, 4.2) << endl;

	cout << "PI: " << PI << endl;

	while (true) {
		cout << "While loop running..." << endl;
		sleep(1);
	}

	return 0;
}

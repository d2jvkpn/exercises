#include <iostream>

using namespace std;

namespace awesome {
	void greetings() {
		cout << "Greetings form awsome namespace!\n";
	}
}

namespace amazing {
	void greetings() {
		cout << "Greetings form amazing namespace!\n";
	}
}

using namespace amazing;


int main() {
	awesome::greetings();
	amazing::greetings();
	greetings();

	return 0;
}

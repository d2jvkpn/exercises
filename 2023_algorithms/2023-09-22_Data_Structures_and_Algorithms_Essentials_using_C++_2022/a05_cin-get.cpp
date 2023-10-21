# include <iostream>

using namespace std;

int main() {
	// cout << "==> 1" << endl;
	// cout << "Hello, world!" << endl;
	//
	// char sentence[100];

	// cin >> sentence;
	// cout << sentence << endl;

	cout << "==> 2" << endl;
	int index = 0;
	char sentence[100];
	char temp;
	while (true) {
		temp = cin.get();
		if (temp == '#') {
			break;
		}
		sentence[index] = temp;
		index++;
		// cout << temp;
	}
	// cout << "==> Char at index and index+1: " << sentence[index] << ", " << sentence[index+1] << endl;
	sentence[index] = '\0';

	cout << "==> Length: " << index << ", sentence: '''" << sentence << "'''" << endl;

	return 0;
}

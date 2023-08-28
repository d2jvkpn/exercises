# include <iostream>

using namespace std;

int main() {
	int nums[3] = {1, 2, 3};

	string strs[3] = {"HELLO", "WORLD", "!"};
	strs[0] = "hello";

	char chars[3];
	chars[0] = 'A';

	cout << nums[0] << endl;
	cout << strs[0] << endl;
	cout << chars[0] << endl;

	int len = sizeof(strs)/sizeof(string);

	for (int i=0; i<len; i++) {
		cout << "--> " << strs[i] << endl;
	}

	return 0;
}

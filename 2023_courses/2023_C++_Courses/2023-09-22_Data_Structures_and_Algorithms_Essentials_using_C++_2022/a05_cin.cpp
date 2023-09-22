# include <iostream>

using namespace std;

int main() {
	char sentence[100];
	int len=0;
	char temp;

	cout << "Enter:" << endl;
	while (temp != '#') { // while (temp != '\n') {
		temp = cin.get();
		sentence[len] = temp;
		len++;
		// cout << temp;
	};
	cout << endl;

	cout << "Length: " << len << endl;
	sentence[len] = '\0';
	cout << "Sentence: " << sentence << endl;

	return 0;
}

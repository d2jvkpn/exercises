#include <iostream>
#include <cstring>

using namespace std;

int main() {
	// cout << "Hello, world!" << endl;

	//
	char route[100];
	int x = 0, y = 0;

	cout << "Enter N, S, E, W: ";
	cin.getline(route, 1000);

	for (int i=0; route[i]!='\0'; i++) {
		switch(route[i]) {
			case 'N':
				y++;
				break;
			case 'S':
				y--;
				break;
			case 'E':
				x++;
				break;
			case 'W':
				x--;
				break;
		}
	}

	cout << "==> Final x and y is: " << x << ", " << y << "." << endl;

	//
	char sentence[1000], largest[1000];
	int n, len = 0, longest = 0;

	cout << "Enter number of sentence: ";	
	cin >> n;
	cin.get(); // cosume \n
	
	cout << "==> Enter sentences:" << endl;
	while (n>0) {
		cin.getline(sentence, 1000);
		len = strlen(sentence);
		if (len > longest) {
			longest = len;
			strcpy(largest, sentence);
		}
		n--;
	}

	cout << "==> Largest sentence: " << largest << endl;

	return 0;
}

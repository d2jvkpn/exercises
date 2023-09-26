# include <iostream>

using namespace std;

int main() {
	// cout << "Hello, world!" << endl;

	char route[100];
	cin.getline(route, 1000);

	int x = 0, y = 0;

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

	return 0;
}

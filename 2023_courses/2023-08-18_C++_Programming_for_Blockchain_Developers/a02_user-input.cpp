# include <iostream>
# include <string>

using namespace std;

int main() {
	string name = "";
	cout << "==> What's your name?\n";
	getline(cin, name); // <string>
	cout << "Good morning, " << name << endl;

	int age = 0;
	cout << "==> How old are you?\n";
	cin >> age;
	cout << "==> Oh so you're " << age << " years old.\n";

	return 0;
}

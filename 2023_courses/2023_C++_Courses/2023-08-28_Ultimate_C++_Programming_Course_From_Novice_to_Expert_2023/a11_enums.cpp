#include <iostream>

using namespace std;

enum Season {
	spring = 0,
	summer = 4,
	autumn = 8,
	winter = 12,
};

enum Num {
	first = 20,
	second,
	third = 50,
};

int main() {
	cout << Season::spring << endl;

	Season season = Season::autumn;
	cout << season << endl;

	cout << Num::second << endl; // 21

	return 0;
}

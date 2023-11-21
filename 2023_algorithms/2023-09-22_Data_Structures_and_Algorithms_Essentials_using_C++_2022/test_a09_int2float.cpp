# include <iostream>

using namespace std;

int main() {
	int v1 = 0;
	cout << "==> Enter: ";
	cin >> v1;

	float v2 = static_cast<float>(v1);
	cout << "v2/2 = " << v2/2.0 << endl;

	return 0;
}

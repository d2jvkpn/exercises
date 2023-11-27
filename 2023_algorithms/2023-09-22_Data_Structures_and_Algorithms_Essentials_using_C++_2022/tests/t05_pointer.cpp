# include <iostream>

using namespace std;

class Data {
public:
	int* value;

	Data(int value) {
		cout << "==> create Data: " << value << endl;
		this->value = &value;
	}

	~Data() {
		cout << "==> delete Data: " << *this->value << endl;
	}

	void show() {
		cout << "--> Data: " << this->value << endl;
	}
};

int main() {
	Data* d1 = new Data(42);
	d1->show();

	int* value = d1->value;
	// delete d1;

	cout << "==> value: " << *value << endl;

	if (d1 != NULL) {
		cout << "YES, d1 isn't NULL after be deleted.\n";
	}
	d1->show();

	return 0;
}

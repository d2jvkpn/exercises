#include <iostream>

using namespace std;

class Data{
	string name;
	int value;

public:
	Data(string name, int value) {
		this->name = name;
		this->value = value;
	}

	void show(){
		cout << "~~ Data: " << name << ", " << value << endl;
	}
};

int main() {
	cout << "Hello, world!\n";

	Data* d1 = new Data("d1", 1);
	Data* d2 = new Data("d2", 2);

	d1->show();
	d2->show();

	swap(d1, d2);

	d1->show();
	d2->show();

	return 0;
}

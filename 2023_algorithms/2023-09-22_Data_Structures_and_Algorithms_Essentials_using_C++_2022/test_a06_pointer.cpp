# include <iostream>

using namespace std;

class Data {
public:
	int   value;
	Data* next;

	Data(int value) {
		cout << "==> create Data: " << value << endl;
		this->value = value;
	}

	~Data() {
		cout << "==> delete Data: " << this->value << endl;
	}

	void clear() {
		if (next != NULL) {
			next->clear();
			delete next;
		}
	}

	void show() {
		if (next == NULL) {
			cout << "--> Data { value: " << this->value << ", next: NULL }" << endl;
		} else {
			cout << "--> Data { value: " << this->value << ", next: " << next->value << " }" << endl;
		}
	}
};

class Struct {
public:
	int value;

	Struct(int value) {
		cout << "==> create Struct: " << value << endl;
		this->value = value;
	}

	~Struct() {
		cout << "==> delete Struct: " << this->value << endl;
	}

	void show() {
		cout << "--> Struct { value: " << this->value << ", next: NULL }" << endl;
	}
};

int main() {
	//
	Data* d1 = new Data(42);
	d1->show();

	Data* d2 = new Data(101);
	d1->next = d2;
	d1->show();
	
	delete d1; // d2 won't be deleted
	d2->show();

	//
	Struct* s1 = new Struct(27);
	s1->show();
	delete s1;

	return 0;
}

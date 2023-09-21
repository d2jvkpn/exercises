# include <iostream>
# include <string>

using namespace std;

class Item {
	public:
		string name;

		Item() {
			cout << "==> Contructor called." << endl;
		}

		void set(string _name) {
			name = _name;
		}

		~Item() {
			cout << "==> Destructor called: " << name << "." << endl;
		}
};

int main() {
	// cout << to_string(42) << endl;

	double *pointer = NULL;
	pointer = new double;

	*pointer = 99.9;
	cout << "Value: " << *pointer << endl;
	delete pointer;

	//
	Item *items = new Item[4];

	for (int i=0; i<4; i++) {
		items[i].set("item " + to_string(i));
	}

	delete[] items;

	return 0;
}

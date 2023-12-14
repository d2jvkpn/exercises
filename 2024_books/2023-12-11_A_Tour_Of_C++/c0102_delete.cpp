#include <iostream>

using namespace std;

class Data {
int id;

public:
	Data(int id): id{id} {}
	Data(): id{0} {};

	void setId(int id) {
		this->id = id;
	}

	~Data() {
		cerr << "!!! delete Data { id: " << id << " }\n";
	}
};

int main() {
	// cout << "Hello, world!\n";
	//
	Data d1(1);
	Data* d2 = new Data(2);

	cout << "==> 1 Delete d2" << endl;
	delete d2; // value must be mannual delete

	//
	Data d3[3] = {{13}, {14}, {15}}; // value will be auto deleted
	cout << "==> 2" << endl;

	//
	Data* d4[2] = {new Data(26), new Data(27)};

	for (int i=0; i<2; i++) {
		delete d4[i]; // must be mannual deleted
	}

	//
	Data* elems = new Data[3];
	elems[0].setId(38);
	elems[1].setId(39);

	delete [] elems;

	return 0;
}

/* Output:
==> 1 Delete d2
!!! delete Data { id: 2 }
==> 2
!!! delete Data { id: 26 }
!!! delete Data { id: 27 }
!!! delete Data { id: 0 }
!!! delete Data { id: 39 }
!!! delete Data { id: 38 }
!!! delete Data { id: 15 }
!!! delete Data { id: 14 }
!!! delete Data { id: 13 }
!!! delete Data { id: 1 }
*/
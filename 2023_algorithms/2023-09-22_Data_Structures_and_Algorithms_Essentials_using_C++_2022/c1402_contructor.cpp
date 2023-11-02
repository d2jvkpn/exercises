# include <iostream>
# include <cstring>

using namespace std;

class Product {
	int id;
	char name[16];
	int mrp;
	int selling_price;

public:
	Product() {
		cout << "==> Construct Product" << endl;
	}

	~Product() {
		cout << "!!! Destruct Product: " << this->name << endl;
	}

	Product(int id, int mrp, int selling_price) {
		this->id = id;
		this->mrp = mrp;
		this->selling_price = selling_price;
	}

	Product(Product &p) {
		id = p.id;
		mrp = p.mrp;
		selling_price = p.selling_price;
		strcpy(name, p.name);
	}

	void show() {
		cout << "--------" << endl;
		cout << "id: " << id << endl;
		cout << "name: " << name << endl;
		cout << "mrp: " << mrp << endl;
		cout << "selling_price: " << selling_price << endl;
	}

	void increaseId() {
		this->id +=1;
	}

	void setName(char name[]) {
		strcpy(this->name, name);
	}

	void setMRP(int val) {
		if (val > 0) {
			mrp = val;
		}
	}

	int getMRP() {
		return mrp;
	}

	void setSellingPrice(int val) {
		if (val > mrp) {
			selling_price = mrp;
		} else {
			selling_price = val;
		}
	}

	int getSellingPrice() {
		return selling_price;
	}
};

int main() {
	Product cam0(101, 28000, 26000);
	Product cam1(cam0);

	cout << "==> 1" << endl;
	cam0.show();
	cam1.show();

	cam1.increaseId();
	char n1[16] = {'c', 'a', 'm', '0', '\0'};
	cam0.setName(n1);

	cout << "==> 2" << endl;
	cam0.show();
	cam1.show();

	cout << "==> 3" << endl;
	Product cam2(cam0);
	char n2[16] = {'c', 'a', 'm', '2', '\0'};
	cam2.setName(n2);
	cam0.show();
	cam2.show();

	/*
	char xx[] = {'H', 'e', 'l', 'l', 'o', '\0'};

	char addr[6];
	strcpy(addr, xx);

	cout << strlen(xx) << ", " << sizeof(xx)/sizeof(char) << endl;

	cout << "xx: " << xx << ", addr: " << addr << endl;
	xx[0] = 'X';
	cout << "xx: " << xx << ", addr: " << addr << endl;
	*/

	return 0;
}

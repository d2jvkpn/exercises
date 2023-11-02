# include <iostream>

using namespace std;

class Car {
	int price;
	int model_id;	

public:
	char name[];

	void show() {
		printf("Name: %s, ModelID %d, Price: %d\n", name, model_id, price);
	}

	void updatePrice(int p) {
		price = p;
	}
};

class Product {
	int id;
	char name[100];

public:
	int mrp;
	int selling_price;

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
	//
	Car car;

	car.show();
	car.updatePrice(4200);
	car.show();

	//
	Product camera;
	cout << "Size of camera: " << sizeof(camera) << endl;

	camera.setMRP(108);
	camera.setSellingPrice(101);

	cout << "MRP: " << camera.mrp << endl;
	cout << "Selling Price: " << camera.selling_price << endl;

	return 0;
}

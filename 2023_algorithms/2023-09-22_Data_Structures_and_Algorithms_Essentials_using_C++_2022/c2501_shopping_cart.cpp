#include <iostream>
#include <sstream>
#include <vector>
#include <iomanip>
#include <unordered_map>

using namespace std;

template <typename T>
using Vector = vector<T>;

using StringStream = stringstream;

template<typename K, typename V>
using UnorderedMap = unordered_map<K, V, hash<K>, equal_to<K>, allocator<pair<const K, V>>>;

void clear() {
	#ifdef _WIN32
		system("cls");
	#else
		system("clear");
	#endif
}

// view product, add product, checkout

class Product {
	int    id;
	string name;
	int    price;

public:
	Product(int u_id, string name, int price) {
		id = u_id;
		this->name = name;
		this->price = price;
	}

	string getName() {
		return name;
	}

	int getId() {
		return id;
	}

	string getShortName() {
		return name.substr(0, 1);
	}

	string toString() {
		StringStream ss;
		ss << "{ id: " << id << ", name: " << quoted(name) << ", price: " << price << " }";

		return ss.str();
	}

	friend class Item; // make sure Item can access all private members
};

class Item {
	Product product;

public:
	int quantity;
	Item(Product p, int q): product(p), quantity(q) {}

	int getId() {
		return product.id;
	}

	int getPrice() {
		return quantity * product.price;
	}

	string toString() {
		return to_string(quantity) + " x " + product.name + " Rs. " + to_string(getPrice());
	}
};

class Cart {
	UnorderedMap<int, Item*> um;

public:
	void add(Product product) {
		int id = product.getId();

		if (um.count(id) == 0) {
			um[id] = new Item(product, 1);
		} else {
			um[id]->quantity += 1;
		}
	}

	int getPrice() {
		int total = 0;

		for (auto pair: um) {
			total += pair.second->getPrice();
		}

		return total;
	}

	bool isEmpty() {
		return um.empty();
	}

	string toString() {
		if (um.empty()) {
			return "!!! Cart is empty";
		}

		int total = 0;
		StringStream ss;

		for (auto pair: um) {
			Item* item = pair.second;
			total += item->getPrice();

			ss << item->toString();
			ss << "\n";
		}

		ss << "Total Amount: Rs. " << total << "\n";

		return ss.str();
	}
};

Vector<Product> allProducts = {
	Product(1, "apple", 26),
	Product(2, "mango", 16),
	Product(3, "guava", 36),
	Product(4, "banana", 56),
	Product(5, "strawberry", 29),
	Product(6, "pineapple", 20),
};

Product* chooseProduct() {
	string products;
	cout << "~~~ Available products:" << endl;

	for (int i=0; i<allProducts.size(); i++) {
		cout << allProducts[i].toString() << endl;
	}

	cout << "--------------------------------" << endl;
	string choice;
	// getline(cin, choice);
	cin >> choice;
	string name;

	for (int i=0; i<allProducts.size(); i++) {
		name = allProducts[i].getShortName();

		if (name == choice || to_string(allProducts[i].getId()) == choice) {
			return &allProducts[i];
		}
	}

	cerr << "!!! Procut not found" << endl;

	return NULL;
}

bool checkout(Cart &cart) {
	if (cart.isEmpty()) {
		return false;
	}

	int total = cart.getPrice();
	int paid;

	cout << "==> Total " << total << ", Pay: ";
	cin >> paid;

	if (paid >= total) {
		cout << "==> Change " << paid - total << endl;
		return true;
	} else {
		cout << "!!! No enough cash" << endl;
		return false;
	}
}

void test01() {
	Product product(1, "apple", 26);
	cout << product.toString() << endl;

	Item fruit(product, 4);
	cout << fruit.getPrice() << endl;
	cout << fruit.toString() << endl;
}

int main() {
	// cout << "Hello, world!\n";
	// test01();

	char action;
	Cart cart;

	while (true) {
		cout << "==> Select an action - (a)dd item, (v)iew cart, (c)heckout" << endl;
		cin >> action;

		switch (action) {
		case 'a': {
			clear();
			Product* product = chooseProduct();
			if (product != NULL) {
				cout << "~~~ Add to the cart: " << product->toString() << endl;
				cart.add(*product);
			}
		} break;
		case 'v': {
			clear();
			cout << "--------------------------------" << endl;
			cout << cart.toString() << endl;
			cout << "--------------------------------" << endl;
		} break;
		case 'c':
			clear();
			if (checkout(cart)) {
				goto end;
				break;
			}
		default:
			break;
		}
	}
	end:

	return 0;
}

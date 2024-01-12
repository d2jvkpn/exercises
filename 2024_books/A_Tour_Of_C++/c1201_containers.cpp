#include <iostream>
#include <vector>

using namespace std;

struct Entry {
	string name;
	int number;
};

ostream& operator<<(ostream& os, const Entry& entry) {
	os << "{ name: \"" << entry.name << "\", number: " << entry.number << " }";
	return os;
}

/*
{ "John Marwood Cleese", 123456 }
{"Michael Edward Palin", 987654    }
*/
istream& operator>>(istream& is, Entry& e) {
	char c1, c2;

	// cin treat space(' ', '\t', '\n') as seperator 
	if (is>>c1 && c1=='{' && is>>c2 && c2 == '"') {
		while (is.get(c1) && c1 != '"') {
			e.name+=c1;
		}

		if (is>>c1 && c1 == ',') {
			if (is>>e.number>>c1 && c1=='}') {
				return is;
			}
		}
	}

	is.setstate(ios_base::failbit);
	return is;
}

void printBookV1(const vector<Entry>& book) {
	for (int i=0; i<book.size(); i++) {
		cout << book[i] << endl;
	}
}

void printBookV2(const vector<Entry>& book) {
	for (const auto& x: book) {
		cout << x << endl;
	}
}

void printBookV3(const vector<Entry>& book) {
	for (auto it = book.begin(); it != book.end(); it++) {
		cout << *it << endl;
	}
}

vector<Entry> inputBook() {
	vector<Entry> book = {};

	for (Entry e; cin>>e;) {
		book.push_back(e);
	}

	return book;
}

int main() {
	// cout << "Hello, world!\n";
	vector<Entry> phone_book = {
		{"David Hume", 123456},
		{"Karl Popper", 234567},
		{"Bertrand Arthur Willian Russell", 123456},
	};

	printBookV1(phone_book);
	printBookV2(phone_book);
	printBookV3(phone_book);

	cout << "==> Enter phone books:" << endl;
	vector<Entry> book = inputBook();
	printBookV3(book);

	return 0;
}

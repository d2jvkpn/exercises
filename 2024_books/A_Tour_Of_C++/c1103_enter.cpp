#include <iostream>

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

int main() {
	// cout << "Hello, world!\n";

	Entry entry;

	cout << "==> Enter Entry: ";
	cin >> entry;
	cout << entry << endl;
	return 0;
}

# include <iostream>

using namespace std;


typedef double height;

struct HomoSapiens {
	string name;
	
};

typedef HomoSapiens Person;

int main() {
	height ivanHeight = 191.0;
	cout << "The height of Ivan " << ivanHeight << ".\n";

	Person person = {"Ivan"};
	cout << "The name is " << person.name << ".\n";

	return 0;
}

# include <iostream>

using namespace std;

struct Person {
	char name[30];
	int age;
	float salary;
};

void display(Person*);

int main() {
	Person person;

	cout << "--> Enter name: ";
	cin.get(person.name, 30);

	cout << "--> Enter age: ";
	cin >> person.age;

	cout << "--> Enter salary: ";
	cin >> person.salary;

	cout << "==> Display all information" << endl;
	// cout << "Name: " << person.name << endl;
	// cout << "Age: " << person.age << endl;
	// cout << "Salary: " << person.salary << endl;
	display(&person);

	return 0;
}

void display (Person *person) {
	cout << "Name: " << person->name << endl;
	cout << "Age: " << person->age << endl;
	cout << "Salary: " << person->salary << endl;	
}

#include <iostream>
#include <vector>

using namespace std;

struct Person {
	string name;
	int age;
	string gender;
};

void printPersonA(Person p) {
	cout << "PrintPersonA: the name of the person is " << p.name << "!\n";
	p.name = "UNKNOWN";
}

void printPersonB(Person *p) {
	cout << "PrintPersonB: the name of the person is " << p->name << "!\n";
	p->name = "UNKNOWN";
}

int main() {
	Person ivan;
	ivan.name = "Ivan";
	ivan.age = 21;
	ivan.gender = "male";

	Person josh;
	josh.name = "Josh";
	josh.age = 27;
	josh.gender = "male";

	Person julia = {"Julia", 24, "female"};

	vector<Person> people = {ivan, josh, julia}; // clone
	cout << "Number of people is " << people.size() << ".\n";

	printPersonA(people[0]); // clone
	cout << "==> The name of the person is " << people[0].name << "!\n";

	printPersonB(&ivan); // referecne
	cout << "==> The name of the person is " << ivan.name << "!\n";

	printPersonB(&people[0]); // referecne
	cout << "==> The name of the person is " << people[0].name << "!\n";

	return 0;
}

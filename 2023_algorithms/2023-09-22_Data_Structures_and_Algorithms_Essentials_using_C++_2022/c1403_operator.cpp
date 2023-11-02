# include <iostream>

using namespace std;

class Person {
public:
	int value;

	Person& operator=(const Person& other) {
		if (this != &other) {
			value = other.value;
		}

		return *this;
	}

	Person() {
		cout << "==> Construct Person\n";
		this->value = 42;
	}

	Person(int value) {
		this->value = value;
	}

	void operator = (Person& p) {
		cout << "~~~ Operator '=': " << p.value << endl;
		this->value = 99;
	}

	void operator += (int value) {
		cout << "~~~ Operator '+='"<< endl;
		this->value += 1;
	}

	void operator ++ (int) {
		cout << "~~~ Operator '++'"<< endl;
		this->value += 1;
	}

	~Person() {
		cout << "!!! Destruct Person: " << this->value << endl;
	}

	void show() {
		cout << "Person { value: " << this->value << " }\n";
	}
};

int main() {
	Person p1;
	Person p2(101);

	// NOT Person p3 = p1;
	Person p3;
	p3 = p1;

	p1.show();
	p2.show();
	p3.show();

	p1+=1;
	p1++;
	p1.show();

	return 0;
}

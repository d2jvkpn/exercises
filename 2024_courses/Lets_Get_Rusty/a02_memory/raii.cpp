// Resource Acquisition Is Initialization
#include <iostream>
#include <fstream>
#include <memory>

using namespace std;

//
class Car{};

void function_that_can_throw() {
	// throw invalid_argument("arguments must be positive");

}

bool should_continue() {
	return false;
}

void memory_example_1() {
	Car* car = new Car;                 // allocate memory on the heap
	function_that_can_throw();          // memory leak if exception is thrown
	if (!should_continue()) { return; } // memory leak if early return
	delete car;                         // clean up memory on the heap
}

void file_example_1() {
	ofstream file("example.txt");       // acquire file handle
	function_that_can_throw();          // file is nerve close 
	if (!should_continue()) { return; } // file is nerver closed if early return
	file.close();                       // close file hanlde
}

//
class CarManager {
private:
	Car* p;
public:
	CarManager(Car* p): p(p) {}
	~CarManager() {
		cerr << "~~~ destruct CarManager" << endl;
		delete p;
	}
};

void memory_example_2() {
	CarManager car = CarManager(new Car);
	function_that_can_throw();
	if (!should_continue()) { return; }
}

class File {
private:
	ofstream file; // file hanlde
public:
	File(string file_name) {
		file = ofstream(file_name);
	}
	~File() {
		file.close();
	}
};

void file_example_2() {
	File file = File("example.txt");
	function_that_can_throw();
	if (!should_continue()) { return; }
}

// RAII
void memory_example_3() {
	unique_ptr<Car> car1 = make_unique<Car>(); // car1 owned to clean up the resouce
	// unique_ptr<Car> car2 = car; // can't compile
	function_that_can_throw();
	if (!should_continue()) { return; }

	shared_ptr<Car> car3 = make_shared<Car>(); // car owned to clean up the resouce
	shared_ptr<Car> car4 = car3; // reference count
}


int main() {
	cout << "Hello, world!\n";

	return 0;
}

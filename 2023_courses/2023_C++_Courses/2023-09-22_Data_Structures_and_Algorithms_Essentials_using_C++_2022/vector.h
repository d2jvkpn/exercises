# include <iostream>

using namespace std;

template<typename T>
class Vector {
	// Data members
	T *array;
	int len, cap;

	// Constructor, Destructor, Methods
public:
	Vector(int _cap=1) {
		len = 0, cap = _cap;
		array = new T[cap];
	}

	~Vector() {
		cout << "--- delete Vector: " << cap << endl;
		delete [] array;
	}

	Vector(int _cap, T val) {
		if (_cap <= 0) {
			len = 0, cap = 1;
		} else {
			len = _cap, cap = _cap;
		}
		array = new T[cap];

		for (int i=0; i<cap; i++) {
			array[i] = val;
		}
	}

	int size() const {
		return len;
	}

	int capacity() const {
		return cap;
	}

	void push_back(T val)  {
		if (len == cap) {
			int* origin = array;
			cap*=2;
			array = new int[cap];
			for (int i=0; i<len; i++) {
				array[i] = origin[i];
			}

			delete [] origin;
		}
		array[len] = val;
		len+=1;
	}

	void pop_back() {
		if (len > 0) {
			len--;
		}
	}

	bool isEmpty() const {
		return len == 0;
	}

	void print() {
		cout << "{ ";
		for (int i=0; i<len; i++) {
			cout << array[i] << ", ";
		}
		cout << "\b\b }" << endl;
	}

	T operator[](const int i) const {
		return array[i];
	}
};

# include <iostream>
# include <vector>

using namespace std;

template <typename T>
class Stack {
public:
	vector<T> vec;

	Stack() {
		vector<T> vec;
		this->vec = vec;
	}

	~Stack() {
		cout << "!!! delete Stack: size=" << this->vec.size();
		cout << ", capacity=" << this->vec.capacity() << endl;
	}

	Stack<T>* push(T val) {
		this->vec.push_back(val);
		return this;
	}

	void pushAtBottom(T val) {
		if (this->isEmpty()) {
			this->push(val);
			return;
		}

		T* ans = this->pop();
		this->pushAtBottom(val);
		this->push(*ans);
	}

	T* pop() {
		int size = this->vec.size();

		if (size == 0) {
			return NULL;
		}

		T* ans = new T;
		*ans = this->vec[size-1];
		this->vec.pop_back();

		return ans;
	}

	void show() {
		cout << "Stack(size=" << this->vec.size() << ", capacity=" << this->vec.capacity();
		cout << ", vec={";

		for (int i=0; i<this->vec.size(); i++) {
			cout << this->vec[i] << ",";
		}
		cout << "\b})\n";
	}

	T* top() {
		int size = this->vec.size();

		if (size == 0) {
			return NULL;
		}

		return &this->vec[size - 1];
	}

	bool isEmpty() {
		return this->vec.size() == 0;
	}
};

int main() {
	// cout << "Hello, world!\n";

	char* ans;

	Stack<char>* stack = new Stack<char>();
	stack->push('a')->push('b')->push('c');
	stack->show();

	ans = stack->pop();
	if (ans != NULL) {
		cout << "~~~ Pop: " << *ans << endl; // c
	} else {
		cout << "~~~ Pop: NULL" << endl;
	}

	stack->pushAtBottom('z');

	ans = stack->top();
	if (ans == NULL) {
		cout << "~~~ Top: NULL" << endl;
	} else {
		cout << "~~~ Top: " << *ans << endl; // b
	}

	stack->show();

	delete stack;

	return 0;
}

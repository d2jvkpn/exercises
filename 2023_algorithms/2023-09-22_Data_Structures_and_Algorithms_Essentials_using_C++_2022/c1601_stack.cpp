# include <iostream>

using namespace std;

template <typename T>
class Node {
public:
	T        data;
	Node<T>* next;

	Node(T data) {
		this->data = data;
	}

	~Node() {
		/*
		if (this->next == NULL) {
			cout << "!!! delete Node(" << this->data << "->NULL)" << endl;
		} else {
			cout << "!!! delete Node(" << this->data << "->" << this->next->data << ")" << endl;
		}
		*/

		this->clear();
		cout << "!!! delete Node(" << this->data << "->NULL)" << endl;
	}

	void show() {
		cout << "Node(" << this->data << "->";

		if (this->next == NULL) {
			cout << "NULL)\n";
			return;
		}

		Node<T>* next = this->next;

		while(next != NULL) {
			cout << next->data << "->";

			if (next->next == NULL) {
				break;
			}

			next = next->next;
		}

		cout << "NULL)\n";
	}

	void push(T data) {
		if (this->next == NULL) {
			this->next = new Node<T>(data);
		} else {
			this->next->push(data);
		}
	}

private:
	void clear() {
		if (this->next != NULL) {
			this->next->clear();
			delete this->next;
			this->next = NULL;
		}
		// delete this; // can't delete self
	}
};

template <typename T>
class Stack {
public:
	int      size;
	Node<T>* head;

	Stack() {
		this->head = NULL;
		this->size = 0;
	}

	~Stack() {
		cout << "!!! delete Stack: " << this->size << endl;
		delete this->head;
	}

	bool isEmpty() {
		return this->head == NULL;
	}

	void clear() {
		cout << "!!! clear Stack" << endl;
		this->size = 0;
		delete this->head;
		this->head = NULL;
	}

	Stack<T>* push(T data) {
		this->size += 1;
		Node<T>* node = new Node<T>(data);
		node->next = this->head;
		this->head = node;

		return this;
	}

	void show() {
		cout << "Stack(size=" << this->size << ", ";

		if (this->head == NULL) {
			cout << "NULL)\n";
			return;
		}

		cout << "Node(" << this->head->data << "->";

		Node<T>* next = this->head->next;

		while(next != NULL) {
			cout << next->data << "->";

			if (next->next == NULL) {
				break;
			}

			next = next->next;
		}

		cout << "NULL))\n";
	}

	Node<T>* top() {
		return this->head;
	}

	Node<T>* pop() {
		if (this->head == NULL) {
			return NULL;
		}

		this->size -= 1;
		Node<T>* ans = this->head;
		this.head = ans->next;
		ans->next = NULL;

		return ans;
	}
};

int main() {
	// cout << "Hello, world!\n";

	///
	Node<int>* node = new Node(42);
	node->push(24);
	node->push(27);
	node->show();

	delete node;

	///
	Stack<int>* stack = new Stack<int>();
	stack->push(0)->push(1)->push(2)->push(3)->push(4);
	stack->show();

	stack->clear();
	stack->show();
	stack->push(5)->push(6)->push(7)->push(9)->push(9);

	delete stack;

	return 0;
}

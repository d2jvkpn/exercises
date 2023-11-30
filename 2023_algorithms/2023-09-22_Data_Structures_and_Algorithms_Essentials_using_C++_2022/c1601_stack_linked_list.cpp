#include <iostream>
// # incliude <"stack.h">
#include "./include/linked_list.h"

using namespace std;

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

		if (this->head != NULL) {
			delete this->head;
		}
	}

	bool empty() {
		return this->head == NULL;
	}

	void clear() {
		cout << "!!! clear Stack" << endl;
		this->size = 0;
		delete this->head;
		this->head = NULL;
	}

	Stack<T>* push(T val) {
		this->size += 1;

		Node<T>* node = new Node<T>(val);
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
	Stack<char>* stack = new Stack<char>();
	stack->push('a')->push('b')->push('c')->push('d')->push('e');
	stack->show();

	stack->clear();
	stack->show();
	stack->push('f')->push('g')->push('h')->push('i')->push('j');

	delete stack;

	return 0;
}

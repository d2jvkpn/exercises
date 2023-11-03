# include <iostream>

using namespace std;

template <typename T>
class Node {
public:
	T        value;
	Node<T>* next;

	Node(T value) {
		this->value = value;
		this->next = NULL;
	}

	~Node() {
		cout << "!!! delete node: " << value << endl;
	}

	void show() {
		cout << "Node(" << this->value << ")\n";
	}
};

template <typename T>
class List {
public:
	Node<T>* head;
	Node<T>* tail;
	int      size;

	List() {
		this->head = NULL;
		this->tail = NULL;
		this->size = 0;
	}

	~List() {
		// TODO:
		cout << "!!! DELETE List: " << this->size << endl;

		this->size = 0;
		this->tail = NULL;

		Node<T>* next = head;
		Node<T>* temp;

		while (next != NULL) {
			temp = next -> next;
			delete next;
			next = temp;
		};
	}

	void show() {
		if (head == NULL) {
			 printf("List(size=%d, NULL, NUULL)\n", size);
			 return;
		}
	
		cout << "List(size=" << size;
		Node<T>* next = head;

		while(true) {
			cout << ", " << next->value;

			if (next->next == NULL) {
				break;
			}

			next = next->next;
		}

		cout << ")\n";
	}

	void push_front(T value) {
		Node<T>*node = new Node(value);
		this->size += 1;

		if (head == NULL) {
			head = node;
			tail = node;
		} else {
			node->next = head;
			head = node;
		}
	}

	void push_back(T value) {
		Node<T>*node = new Node(value);
		this->size += 1;

		if (head == NULL) {
			head = node;
			tail = node;
		} else {
			tail->next = node;
			tail = node;
		}
	}

	void insert(T value, int pos) {
		// assert!(pos >= 0)
		this->size += 1;

		if (size == 0 || pos == 0) {
			push_front(value);
			return;
		} else if (pos >= size-1) {
			push_back(value);
			return;
		}

		Node<T>* node = new Node(value);
		Node<T>* next = head;

		for (int i=0; i<pos-1; i++) {
			next = next->next;
		}

		node->next = next->next;
		next->next = node;
	}

	int search(T value) {
		Node<T>* next = head;
		int i= 0;

		while (next != NULL) {
			if (next->value == value) {
				return i;
			}

			next = next->next;
			i+=1;
		}

		return -1;
	}

	bool remove(T value) {
		if (this->size == 0) {
			return false;
		}

		Node<T>* next = head;
		Node<T>* parent;

		while (next != NULL) {
			if (next->value == value) {
				break;
			}

			parent = next;
			next = next -> next;
		}

		if (next == NULL) {
			return false;
		}

		this->size -= 1;
		if (this->size == 0) { // parent == NULL
			this->head = NULL;
			this->tail = NULL;
		} else {
			parent->next = next->next;
		}

		if (parent->next == NULL) {
			this->tail = parent;
		}

		next->next = NULL;

		return true;
	}

	Node<T>* pop_back() {
		Node<T>* ans;

		if (this->size == 0) {
			return NULL;
		}

		ans = this->tail;
		this->size -= 1;

		if (this->size == 0) {
			this->head = NULL;
			this->tail = NULL;
		} else {
			Node<T>* next = this->head;

			while (next->next != this->tail) {
				next = next->next;
			}

			this->tail = next;
		}

		ans->next = NULL;

		return ans;
	}

	Node<T>* pop_front() {
		Node<T>* ans;

		if (this->size == 0) {
			return NULL;
		}

		ans = this->head;
		this->size -= 1;

		if (this->size == 0) {
			this->head = NULL;
			this->tail = NULL;
		} else {
			this->head = this->head->next;
		}

		ans->next = NULL;

		return ans;
	}

	void reverse() {
		if (this->size <= 1) {
			return;
		}

		Node<T>* current = head;
		Node<T>* prev = NULL;
		Node<T>* temp;

		while (current != NULL) {
			temp = current->next;
			current->next = prev;

			prev = current;
			current = temp;
		}

		tail = head;
		head = prev;
	}
};

int main() {
	List<int> list;
	list.show();

	list.push_back(42);
	list.show();

	list.push_back(101);
	list.show();

	list.push_back(102);
	list.push_back(103);
	list.push_front(41);
	list.show();

	list.insert(199, 2);
	list.show();

	cout << "==> Search 33: " << list.search(33) << endl;
	cout << "==> Search 199: " << list.search(199) << endl;

	list.show();
	list.reverse();
	list.show();

	return 0;
}

# include <iostream>

using namespace std;

template <typename T>
class Node {
public:
	T        data;
	Node<T>* next;

	Node(T data) {
		this->data = data;
		this->next = NULL;
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

	void push(T data) {
		if (this->next == NULL) {
			this->next = new Node<T>(data);
		} else {
			this->next->push(data);
		}
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
class List {
public:
	Node<T>* head;
	Node<T>* tail;
	int      size;

	// List(): head(NULL), tail(NULL) {}

	List() {
		this->head = NULL;
		this->tail = NULL;
		this->size = 0;
	}

	~List() {
		cout << "!!! DELETE List: " << this->size << " nodes" << endl;

		if (this->size > 0) {
			this->tail = NULL;
			delete this->head;
		}
	}

	void show() {
		if (this->head == NULL) {
			 printf("List(size=%d, NULL)\n", size);
			 return;
		}
	
		cout << "List(size=" << size << ", ";
		Node<T>* next = this->head;

		while(next != NULL) {
			cout << next->data << "->";

			if (next->next == NULL) {
				break;
			}

			next = next->next;
		}

		cout << "NULL)\n";
	}

	void push_front(T data) {
		Node<T>*node = new Node(data);
		this->size += 1;

		if (head == NULL) {
			head = node;
			tail = node;
		} else {
			node->next = head;
			head = node;
		}
	}

	void push_back(T data) {
		Node<T>*node = new Node(data);
		this->size += 1;

		if (head == NULL) {
			head = node;
			tail = node;
		} else {
			tail->next = node;
			tail = node;
		}
	}

	void insert(T data, int pos) {
		// assert!(pos >= 0)
		this->size += 1;

		if (size == 0 || pos == 0) {
			push_front(data);
			return;
		} else if (pos >= size-1) {
			push_back(data);
			return;
		}

		Node<T>* node = new Node(data);
		Node<T>* next = head;

		for (int i=0; i<pos-1; i++) {
			next = next->next;
		}

		node->next = next->next;
		next->next = node;
	}

	int search(T data) {
		Node<T>* next = head;
		int i= 0;

		while (next != NULL) {
			if (next->data == data) {
				return i;
			}

			next = next->next;
			i+=1;
		}

		return -1;
	}

	bool remove(T data) {
		if (this->size == 0) {
			return false;
		}

		Node<T>* next = head;
		Node<T>* parent;

		while (next != NULL) {
			if (next->data == data) {
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

			next->next = NULL; // ! must do this
			this->tail = next;
		}

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
			this->head = ans->next;
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

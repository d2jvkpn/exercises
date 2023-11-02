# include <iostream>

using namespace std;

template <typename T>
class Node {
public:
	T     data;
	Node<T>* next;

	Node(T data) {
		this->data = data;
		this->next = NULL;
	}

	void show() {
		cout << "Node(" << this->data << ")\n";
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
	}

	void show() {
		if (head == NULL) {
			 printf("List(size=%d, NULL, NUULL)\n", size);
			 return;
		}
	
		cout << "List(size=" << size;
		Node<T>* next = head;

		while(true) {
			cout << ", " << next->data;

			if (next->next == NULL) {
				break;
			}

			next = next->next;
		}

		cout << ")\n";
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
	list.show();

	list.insert(199, 2);
	list.show();

	return 0;
}

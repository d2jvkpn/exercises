#include <iostream>

using namespace std;

struct Node {
	int data;
	struct Node* next;

	Node(int data) {
		this->data = data;
		next = NULL;
	}
};

struct LinkedList {
	Node* head;

	LinkedList() {
		head = NULL;
	}

	void print() {
		Node* node = head;
		
		while (node != NULL) {
			cout << node->data << "->";
			node = node->next;
		}

		cout << endl;
	}

	void prepend(int data) {
		Node* node = new Node(data);
		node->next = head;
		head = node;
	}

	void push(int data) {
		Node* node = head;
		if (node == NULL) {
			head = new Node(data);
			return;
		}

		while (node->next != NULL) {
			node = node->next;
		}

		node->next = new Node(data);
	}

	void reverse() {
		Node* node = head;
		Node *prev = NULL, *next = NULL;

		while (node != NULL) {
			next = node->next;
			node->next =  prev;
			prev = node;
			node = next;
		}

		head = prev;
	}
};

int main() {
	LinkedList ll;
	ll.push(1);
	ll.push(2);
	ll.push(3);

	ll.print();
	ll.reverse();
	ll.print();

	return 0;
}

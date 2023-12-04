#include <iostream>
#include <array>

using namespace std;

template <typename T>
class Node {
public:
	string key;
	T      value;
	Node*  next;

	Node(string key, T value) {
		this->key = key;
		this->value = value;
		this->next = NULL;
	}

	~Node() {
		// next->clear();
		if (this->next != NULL) {
			delete this->next;
			this->next = NULL;
		}
		cout << "!!! delete Node: key=\"" << key << "\", value=" << value << endl;
	}

	array<Node<T>*, 2> find(string key) {
		Node<T>* temp = this;

		while (temp != NULL) {
			if (temp->key == key) {
				return {NULL, temp};
			}

			if (temp->next != NULL && temp->next->key == key) {
				return {temp, temp->next};
			}

			temp = temp->next;
		}

		return {NULL, NULL};
	}
};

template <typename T>
class HashTable {
	Node<T>** table; // an array of Node<T>*
	int       cs, ts;
	// cs/ts = 0.7~0.8

private:
	int hashFn(string key) {
		int idx = 0, power = 1;

		for (auto c : key) {
			idx = (idx + c*power)%ts;
			power = (power*29)%ts;
		}

		return idx;
	}

	void rehash() {
		int oldTs = ts;

		if (cs * 100 > ts * 80) { // greater than 80% of capacity
			ts = 2*ts + 1;
		} else if (cs > 32 && cs * 100 < ts * 25) { // less than 25% of capacity and capcity > 32
			ts = ts/2+1;
		} else { // don't do anything
			return;
		}

		cs = 0; // must reset cs
		Node<T>** oldTable = table;
		table = new Node<T>*[ts];
		Node<T>* temp;

		/*
		for (int i=0; i<ts; i++) {
			table[i] = NULL;
		}
		*/

		for (int i=0; i<oldTs; i++) {
			temp = oldTable[i];
			if (temp == NULL) {
				continue;
			}

			while (temp != NULL) {
				insert(temp->key, temp->value); // children may not have the same idx
				temp = temp->next;
			}
		}

		delete [] oldTable;
	}

public:
	HashTable(int default_size=7) {
		cs = 0, ts = default_size;
		table = new Node<T>*[ts];

		/*
		for (int i=0; i<ts; i++) {
			table[i] = NULL;
		}
		*/
	}

	void insert(string key, T value) {
		int idx = hashFn(key);
		array<Node<T>*, 2> pair = table[idx]->find(key);

		if (pair[1] == NULL) {
			pair[1] = new Node<T>(key, value);
			pair[1]->next = table[idx];
			table[idx] = pair[1];
			cs+=1;
		} else {
			pair[1]->value = value;
		}

		rehash();
	}

	bool remove(string key) {
		int idx = hashFn(key);
		array<Node<T>*, 2> pair = table[idx]->find(key);

		if (pair[1] == NULL) {
			return false;
		} else if (pair[0] == NULL) {
			table[idx] = pair[1]->next;
			pair[1]->next = NULL;
		} else {
			pair[0] = pair[1]->next;
			pair[1]->next = NULL;
		}

		delete pair[1];
		cs-=1;
		rehash();

		return true;
	}

	bool has(string key) {
		int idx = hashFn(key);
		Node<T>* temp = table[idx];

		while (temp!=NULL) {
			if (temp->key == key) {
				return true;
			}

			temp = temp->next;
		}

		return false;
	}

	T* search(string key) {
		int idx = hashFn(key);
		Node<T>* temp = table[idx];

		while (temp!=NULL) {
			if (temp->key == key) {
				return &temp->value;
			}

			temp = temp->next;
		}

		return NULL;
	}

	void show() {
		printf("==> HashTable: ts=%d, cs=%d\n", ts, cs);

		for (int i=0; i<ts; i++) {
			Node<T> *temp = table[i];

			if (temp == NULL) {
				continue;
			}

			cout << "Bucket [" << i << "]: ";

			while (temp != NULL) {
				cout << temp->key << "(" << temp->value << ")->";
				temp = temp->next;
			}

			cout << endl;
		}
	}

	T* operator[] (string key) {
		T* item = search(key);

		if (item == NULL) {
			T object;
			insert(key, object);
			item = search(key);
		}

		return item;
	}
};

int main() {
	// cout << "Hello, world!\n";
	HashTable<int> ht;

	ht.insert("Mango", 100);
	ht.insert("Apple", 120);
	ht.insert("Banana", 140);
	ht.insert("Orange", 200);
	ht.insert("Kiwi", 210);
	ht.insert("Papaya", 220);
	ht.insert("Litchi", 30);
	ht.show();

	ht.remove("Kiwi");
	ht.show();

	ht.insert("Kiwi", 333);
	ht.show();

	cout << "==> Enter fruit: ";
	string fruit;
	getline(cin, fruit); // cin >> fruit;

	int* ans = ht.search(fruit);
	if (ans != NULL) {
		cout << "Ans: " << *ans << endl;
	} else {
		cerr << "NotFound: " << fruit << endl;
	}

	return 0;
}

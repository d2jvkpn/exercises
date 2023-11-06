# include <iostream>

using namespace std;

class Queue{
	int *arr;

	int max_size;
	int size;

	int front;
	int rear;

public:
	Queue(int size=5) {
		this->arr = new int[size];

		this->max_size = size;
		this->size = 0;

		this->front = 0;
		this->rear = size - 1;
	}

	bool full() {
		return this->size == this->max_size;
	}

	bool empty() {
		return this->size == 0;
	}

	void show() {
		if (this->empty()) {
			cerr << "~~~ Queue is Empty" << endl;
		} else {
			cout << "~~~ Queue: ";

			for (int i=this->front; i<this->size; i++) {
				cout << this->arr[i%this->max_size] << "<-";
			}
			cout << endl;
		}
	}

	Queue* push(int val) {
		if (!this->full()) {
			this->rear = (this->rear + 1) % this->max_size;
			this->arr[this->rear] = val;
			this->size+=1;
		} else {
			cerr << "!!! The queue is full: " << val << endl;
		}

		return this;
	}

	void debug() {
		printf(
			"DEBUG: max_size=%d, size=%d, front=%d, rear=%d\n",
			this->max_size, this->size, this->front, this->rear
		);
	}

	int* pop() {
		int* ans = new int;

		if (!this->empty()) {
			// cout << "~~~" << this->arr[this->front] << endl;
			*ans = this->arr[this->front];

			this->front = (this->front + 1) % this->max_size;
			this->size-=1;
			return ans;
		} else {
			cerr << "!!! The queue is empty" << endl;
			return NULL;
		}
	}

	int* getFront() {
		int* ans = new int;

		if (!this->empty()) {
			*ans = this->arr[this->front];

			return ans;
		} else {
			cerr << "!!! The queue is empty" << endl;
			return NULL;
		}
	}
};

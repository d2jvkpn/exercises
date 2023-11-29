#include <iostream>
#include <vector>

using namespace std;

template<typename T>
class PriorityQueue {
private:
	vector<T> data;
	bool      isMin;

	bool compare(int parent, int child) {
		if (isMin) {
			return data[parent] <= data[child];
		} else {
			return data[parent] >= data[child];
		}
	}

	void heapifyUp(int index) {
		if (index < 1) {
			return;
		}

		int parent = (index - 1) / 2;

		if (!compare(parent, index)) {
			swap(data[parent], data[index]);
			heapifyUp(parent);
		}
	}

	void heapifyDown(int parent) {
		int left = 2*parent + 1;
		int right = 2*parent + 2;
		int target = parent;

		if (left > data.size() - 1) {
			return;
		}

		if (!compare(target, left)) {
			target = left;
		}

		if (right < data.size() && !compare(target, right)) {
			target = right;
		}

		if (target != parent) {
			swap(data[target], data[parent]);
			heapifyDown(target);
		}
	}

public:
	PriorityQueue(int size, bool isMin) {
		this->data.reserve(size);
		this->isMin = isMin;
	}

	bool empty() {
		return data.empty();
	}

	T top() {
		if (!data.empty()) {
			return data.front();
		} else {
			throw out_of_range("Heap is empty");
		}
	}

	PriorityQueue<T>* push(T value) {
		data.push_back(value);
		heapifyUp(data.size() - 1);

		return this;
	}

	void push_vector(vector<T>& vec) {
		for (int i=0; i<vec.size(); i++) {
			data.push_back(vec[i]);
			heapifyUp(data.size() - 1);
		}
	}

	void pop() {
		if (!data.empty()) {
			data[0] = data.back();
			data.pop_back();
			heapifyDown(0);
		} else {
			throw out_of_range("Heap is empty");
		}
	}

	void show() {
		if (isMin) {
			cout << "Min-Heap Priority Queue: { ";
		} else {
			cout << "Max-Heap Priority Queue: { ";
		}

		while (!empty()) {
			cout << top() << ", ";
			pop();
		}
		cout << "\b\b }" << endl;
	}
};

int main() {
	//
	// PriorityQueue<int> pq1(10, false);
	PriorityQueue<int>* pq1 = new PriorityQueue<int>(10, false);

	pq1->push(3)->push(2)->push(15)->push(5)->push(4)
	  ->push(45);

	pq1->show();

	//
	PriorityQueue<int>* pq2 = new PriorityQueue<int>(10, true);

	pq2->push(9)->push(7)->push(5)->push(11)->push(12)
	  ->push(2)->push(14)->push(3)->push(10)->push(6);

	pq2->show();

	//
	PriorityQueue<int>* pq3 = new PriorityQueue<int>(10, false);

	pq3->push(9)->push(7)->push(5)->push(11)->push(12)
	  ->push(2)->push(14)->push(3)->push(10)->push(6);

	pq3->show();

	return 0;
}

# include <iostream>
# include <vector>

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

	void heapify_up(int i) {
		if (i <= 0) {
			return;
		}

		int parent = (i - 1) / 2;

		if (!compare(parent, i)) {
			swap(data[parent], data[i]);
			heapify_up(parent);
		}
	}

	void heapify_down(int i) {
		int left = 2 * i + 1;
		int right = 2 * i + 2;
		int max = i;

		if (left > data.size() - 1) {
			return;
		}

		if (!compare(max, left)) {
			max = left;
		}

		if (right < data.size() && !compare(max, right)) {
			max = right;
		}

		if (max != i) {
			swap(data[max], data[i]);
			heapify_down(max);
		}
	}

public:
	PriorityQueue(int size, bool isMin) {
		this->data.reserve(size);
		this->isMin = isMin;
	}

	bool empty() const {
		return data.empty();
	}

	T top() const {
		if (!data.empty()) {
			return data.front();
		} else {
			throw out_of_range("Heap is empty");
		}
	}

	void push(T value) {
		data.push_back(value);
		heapify_up(data.size() - 1);
	}

	void pop() {
		if (!data.empty()) {
			data[0] = data.back();
			data.pop_back();
			heapify_down(0);
		} else {
			throw out_of_range("Heap is empty");
		}
	}

	void show() {
		if (isMin) {
			cout << "Min-Heap PriorityQueue: { ";
		} else {
			cout << "Max-Heap PriorityQueue: { ";
		}

		for (int i=0; i<data.size(); i++) {
			cout << data[i] << ", ";
		}
		cout << "\b\b }" << endl;
	}
};

int main() {
	PriorityQueue<int> pq(10, false);

	pq.push(3);
	pq.push(2);
	pq.push(15);
	pq.push(5);
	pq.push(4);
	pq.push(45);

	pq.show();

	return 0;
}

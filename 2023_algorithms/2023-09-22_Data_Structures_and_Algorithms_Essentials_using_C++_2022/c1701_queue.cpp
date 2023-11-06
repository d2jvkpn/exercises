# include <iostream>
# include "c1701_queue.h"

using namespace std;

int main() {
	// cout << "Hello, world!\n";

	int* ans;
	Queue* queue = new Queue(7);

	queue->push(1)->push(2)->push(3)->push(4)->push(5)->push(6)->push(7)->push(8)->push(9);

	queue->debug();
	queue->show();

	ans = queue->getFront();
	cout << "--> Front: " << *ans << endl;

	while(!queue->empty()) {
		ans = queue->pop();
		cout << "--> Pop: " << *ans << endl;
	}

	ans = queue->pop();

	return 0;
}

#include <iostream>
#include <queue>

using namespace std;

void show_queue(queue<int> rq) {
	queue<int> g = rq;

	while (!g.empty()) {
		cout << ',' << g.front();
		g.pop();
	}
	
	cout << endl;
}

int main() {
	queue<int> q1;
	q1.push(25);
	q1.push(77);
	q1.push(33);

	cout << "Queue = ";
	show_queue(q1);
	cout << endl;
	
	cout << "Size = " << q1.size() << endl;
	cout << "Front = " << q1.front() << endl;
	cout << "Back = " << q1.back() << endl;

	return 0;
}

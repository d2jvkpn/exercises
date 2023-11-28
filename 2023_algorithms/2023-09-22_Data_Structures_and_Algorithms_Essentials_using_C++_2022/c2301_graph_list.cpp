# include <iostream>
# include <list>

using namespace std;

class Graph {
	int size;
	list<int>* data;

public:
	Graph(int size) {
		this->size = size;
		this->data = new list<int>[size];
	}

	~Graph() {
		cout << "!!! delete graph" << endl;
	}

	Graph* addEdge(int i, int j, bool undirected=true) {
		data[i].push_back(j);
	
		if (undirected) {
			data[j].push_back(i);
		}

		return this;
	}

	void show() {
		for (int i=0; i<this->size; i++) {
			cout << i << " ->\n    ";
			for (auto node: data[i]) {
				cout << node << ", ";
			}
			cout << endl;
		} 
	}
};

int main() {
	// cout << "Hello, world!\n";

	Graph graph(6);
	graph.addEdge(0, 1);
	graph.addEdge(0, 4);
	graph.addEdge(2, 1);
	graph.addEdge(3, 4);
	graph.addEdge(4, 5);
	graph.addEdge(2, 3);
	graph.addEdge(3, 5);

	graph.show();

	return 0;
}

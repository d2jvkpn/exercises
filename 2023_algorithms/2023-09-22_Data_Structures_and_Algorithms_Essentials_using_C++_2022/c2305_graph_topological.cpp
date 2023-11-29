# include <iostream>
# include "lib/ns01.h"

using namespace std;
using namespace ns01;

template <typename T>
class Graph {
	string            name;
	int               size;
	Vector<Vector<T>> data;

public:
	Graph(string name, int size) {
		this->name = name;
		this->size = size;

		Vector<T> item;
		item.reserve(2);
		this->data.assign(size, item);

		/*
		for (int i=0; i<size; i++) {
			data.push_back(item); // push a clone of vector item
		}
		*/
	}

	~Graph() {
		cout << "!!! delete graph: name=\"" << name << "\", size=" << size << endl;
	}

	// bool addEdge(int i, int j, bool undirected=true) {
	bool addEdge(int i, int j, bool undirected) {
		if (i >= size || j >= size) {
			return false;
		}

		data[i].push_back(j);
	
		if (undirected) {
			data[j].push_back(i);
		}

		return true;
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

	void bfs(int source) {
		Queue<int> queue;
		bool *visited = new bool[this->size] {false};

		queue.push(source);
		visited[source] = true;

		cout << "==> BFS:" << endl;
		while(!queue.empty()) {
			int node = queue.front();
			queue.pop();

			cout << "  goto: " << node << endl;
			cout << "  CALL: " << node << endl;
			visited[node] = true;

			for (auto nbr: data[node]) {
				if (!visited[nbr]) {
					queue.push(nbr);
				}
			}
		}
		cout << endl;
	}

	void dfs(int source) {
		bool* visited = new bool[size]{0};

		cout << "==> DFS:" << endl;
		dfsRecur(source, visited);
		cout << endl;

		delete [] visited;
	}

	void dfsRecur(int node, bool* visited) {
		cout << "  goto: " << node << endl;
		visited[node] = true;

		for (int nbr: data[node]) {
			if (!visited[nbr]) {
				dfsRecur(nbr, visited);
			}
		}

		cout << "  CALL: " << node << endl;
	}

	void topologicalSort() {
		cout << "==> Topological Sort:" << endl;
		Vector<int> indegree(this->size, 0);

		for (int i=0; i<this->size; i++) {
			for (auto nbr: this->data[i]) {
				indegree[nbr]++;
			}
		}

		cout << "--> indegree:" << endl;
		for (int i=0; i<this->size; i++) {
			cout << "  " << i << ": " << indegree[i] << endl;
		}

		Queue<int> queue;
		for (int i=0; i<this->size; i++) {
			if (indegree[i] == 0) {
				queue.push(i);
				cout << "  push: " << i << endl;
			}
		}

		while (!queue.empty()) {
			int node = queue.front();
			queue.pop();

			cout << "  CALL: " << node << endl;
			for (auto nbr: this->data[node]) {
				indegree[nbr]--;
				if (indegree[nbr] == 0) {
					queue.push(nbr);
					cout << "  push: " << nbr << endl;
				}
			}
		}
		cout << endl;
	}
};

int main() {
	// cout << "Hello, world!\n";

	//
	Graph<int> g1("g1", 7);
	g1.addEdge(0, 1, true);
	g1.addEdge(1, 2, true);
	g1.addEdge(3, 5, true);
	g1.addEdge(5, 6, true);
	g1.addEdge(4, 5, true);
	g1.addEdge(0, 4, true);
	g1.addEdge(3, 4, true);

	g1.show();
	g1.bfs(1);
	g1.dfs(1);

	//
	Graph<int> g2("g2", 6);
	g2.addEdge(0, 2, false);
	g2.addEdge(2, 3, false);
	g2.addEdge(3, 5, false);
	g2.addEdge(4, 5, false);
	g2.addEdge(1, 4, false);
	g2.addEdge(1, 2, false);

	g2.show();
	g2.topologicalSort();

	//

	return 0;
}

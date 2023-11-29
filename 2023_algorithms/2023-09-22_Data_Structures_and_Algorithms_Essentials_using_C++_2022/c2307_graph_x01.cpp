# include <iostream>
# include <climits>
# include <sstream>

# include "lib/ns01.h"

using namespace std;
using namespace ns01;

class Graph {
	string  name;
	int     size;

	Vector<Vector<Pair<int, double>>> data;

public:
	//
	Graph(string name, int size) {
		this->name = name;
		this->size = size;

		Vector<Pair<int, double>> item;
		item.reserve(2);
		this->data.assign(size, item); // push a clone of item to data
	}

	~Graph() {
		cout << "!!! Delete Graph: name=\"" << name << "\", size=" << size << endl;
	}

	string describe() {
		int edges = 0;
		for (int i=0; i<size; i++) {
			edges += data[i].size();
		}

		stringstream ss;
		ss << "name=\"" << name << "\", nodes=" << size << ", edges=" << edges;

		return ss.str();
	}

	void show() {
		cout << "==> Graph: " << describe() << endl;

		for (int i=0; i<this->size; i++) {
			cout << i << " ->";

			for (auto pair: data[i]) {
				cout << " { nbr:" << pair.first << ", weight:" << pair.second << " };";
			}
			cout << endl;
		}

		cout << endl;
	}

	//
	bool addEdge(int u, int v, bool undirected) {
		if (u > size-1 || v > size-1) {
			return false;
		}

		data[u].push_back({v, 0});

		if (undirected) {
			data[v].push_back({u, 0});
		}

		return true;
	}

	bool addEdge(int u, Pair<int, double> edge, bool undirected) {
		if (u > size-1 || edge.first > size - 1) {
			return false;
		}

		data[u].push_back(edge);

		if (undirected) {
			data[edge.first].push_back({u, edge.second});
		}

		return true;
	}

	//
	void bfs(int source) {
		cout << "==> BFS: source=" << source << endl;

		Queue<int> queue;
		bool *visited = new bool[this->size] {false};

		queue.push(source);
		cout << "~ push: " << source << ", ";
		visited[source] = true;

		while(!queue.empty()) {
			int node = queue.front();
			queue.pop();
			cout << "~ pop: " << node << ", ";

			cout << "~ goto: " << node << ", ";
			cout << "CALL: " << node << endl;
			visited[node] = true;

			for (auto pair: data[node]) {
				if (!visited[pair.first]) {
					queue.push(pair.first);
					cout << "~ push: " << pair.first << ", ";
				}
			}
		}

		cout << endl;
	}

	//
	void dfs(int source) {
		bool* visited = new bool[size]{0};

		cout << "==> DFS: source=" << source << endl;

		dfsRecur(source, visited);
		cout << endl;

		delete [] visited;
	}

	void dfsRecur(int node, bool* visited) {
		cout << "~ goto: " << node << ", ";
		visited[node] = true;

		for (auto pair: data[node]) {
			if (!visited[pair.first]) {
				dfsRecur(pair.first, visited);
			}
		}

		cout << "CALL: " << node << endl;
	}

	//
	void topologicalSort() {
		cout << "==> Topological Sort:" << endl;
		Vector<int> indegree(this->size, 0);

		for (int i=0; i<this->size; i++) {
			for (auto pair: this->data[i]) {
				indegree[pair.first]++;
			}
		}

		cout << "--> indegree: ";
		for (int i=0; i<this->size; i++) {
			cout << i << ": " << indegree[i] << ", ";
		}
		cout << endl;

		Queue<int> queue;
		for (int i=0; i<this->size; i++) {
			if (indegree[i] == 0) {
				queue.push(i);
				cout << "~ push: " << i << ", ";
			}
		}

		while (!queue.empty()) {
			int node = queue.front();
			queue.pop();
			cout << "~ pop: " << node << ", ";

			cout << "CALL: " << node << endl;
			for (auto pair: this->data[node]) {
				indegree[pair.first]--;
				if (indegree[pair.first] == 0) {
					queue.push(pair.first);
					cout << "~ push: " << pair.first << ", ";
				}
			}
		}
		cout << endl;
	}

	//
	int dijkstra(int src, int dest) {
		cout << "==> dijkstra: " << name << endl;

		Vector<bool>        dist(size, INT_MAX);
		Set<Pair<int, int>> set;

		dist[src] = 0;
		set.insert({0, src});

		while (!set.empty()) {
			auto item = set.begin();
			int node = item->first;
			double wt1 = item->second;

			set.erase(item); // pop

			for (auto pair : data[node]) {
				int nbr = pair.first;
				double wt2 = pair.second;

				if (wt1 + wt2 < dist[nbr]) {
					auto pair = set.find({dist[nbr], nbr});
					if (pair != set.end()) {
						set.erase(pair);
					}

					dist[nbr] = wt1 + wt2;
					set.insert({dist[nbr], nbr});
				}
			}
		}

		for (int i=0; i<size; i++) {
			cout << "~ Node: " << i << ", dist=" << dist[i] << endl;
		}
		cout << endl;

		return dist[dest];
	}
};

int main() {
	// cout << "Hello, world!\n";

	//
	Graph g1("g1", 7);
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
	Graph g2("g2", 6);
	g2.addEdge(0, 2, false);
	g2.addEdge(2, 3, false);
	g2.addEdge(3, 5, false);
	g2.addEdge(4, 5, false);
	g2.addEdge(1, 4, false);
	g2.addEdge(1, 2, false);

	g2.show();
	g2.topologicalSort();

	//
	Graph g3("g3", 5);

	g3.addEdge(0, {1, 1.0}, true);
	g3.addEdge(1, {2, 1.0}, true);
	g3.addEdge(0, {2, 4.0}, true);
	g3.addEdge(0, {3, 7.0}, true);
	g3.addEdge(3, {2, 2.0}, true);
	g3.addEdge(3, {4, 3.0}, true);

	g3.show();

	double ans = g3.dijkstra(0, 4);
	cout << "==> Dijkstra(0, 4): " << ans << endl;

	return 0;
}

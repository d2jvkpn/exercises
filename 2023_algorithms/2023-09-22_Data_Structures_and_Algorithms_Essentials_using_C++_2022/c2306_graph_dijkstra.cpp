# include <iostream>
# include <climits>
# include "lib/ns01.h"

using namespace std;
using namespace ns01;

template <typename T>
class Graph {
	int                          size;
	Vector<Vector<pair<int, T>>> data;

public:
	Graph(int size) {
		this->size = size;

		Vector<pair<int, T>> item;
		item.reserve(2);
		this->data.assign(size, item);
	}

	void addEdge(int u, int v, int wt, bool undirected) {
		data[u].push_back({wt, v});

		if (undirected) {
			data[v].push_back({wt, u});
		}
	}

	int dijkstra(int src, int dest) {
		Vector<bool>        dist(size, INT_MAX);
		Set<pair<int, int>> set;

		dist[src] = 0;
		set.insert({0, src});

		while (!set.empty()) {
			auto item = set.begin();
			int node = item->second;
			int d1 = item->first;

			set.erase(item); // pop

			for (auto pair : data[node]) {
				int nbr = pair.second;
				int d2 = pair.first;

				if (d1 + d2 < dist[nbr]) {
					auto f = set.find({dist[nbr], nbr});
					if (f != set.end()) {
						set.erase(f);
					}

					dist[nbr] = d1 + d2;
					set.insert({dist[nbr], nbr});
				}
			}
		}

		for (int i=0; i<size; i++) {
			cout << "Node: " << i << ", dist=" << dist[i] << endl;
		}

		return dist[dest];
	}
};

int main() {
	// cout << "Hello, world!\n";

	Graph<int> g1(5);
	g1.addEdge(0, 1, 1, true);
	g1.addEdge(1, 2, 1, true);
	g1.addEdge(0, 2, 4, true);
	g1.addEdge(0, 3, 7, true);
	g1.addEdge(3, 2, 2, true);
	g1.addEdge(3, 4, 3, true);

	cout << g1.dijkstra(0, 4) << endl;
	return 0;
}

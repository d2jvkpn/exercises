# include <iostream>
# include <vector>
# include <queue>

using namespace std;

template <typename T>
class Graph {
	string            name;
	int               size;
	vector<vector<T>> data;

public:
	Graph(string name, int size) {
		this->name = name;
		this->size = size;

		vector<T> item;
		item.reserve(2);
		this->data.assign(size, item);

		/*
		for (int i=0; i<size; i++) {
			data.push_back(item); // push a clone of vector item
		}
		*/
	}

	~Graph() {
		cout << "!!! delete graph: " << name << ", size=" << size << endl;
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
        queue<int> q;
        bool *visited = new bool[this->size] {false};

        q.push(source);
		visited[source] = true;

		cout << "==> BFS:" << endl;
        while(!q.empty()) {
            int f = q.front();
			cout << "  goto: " << f << endl;
            cout << "  CALL: " << f << endl;
            q.pop();

            for (auto nbr: data[f]) {
                if (visited[nbr]) {
                    continue;
                }
                q.push(nbr);
                visited[nbr] = true;
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
		visited[node] = true;

		cout << "  goto: " << node << endl;

		for (int nbr: data[node]) {
			if (!visited[nbr]) {
				dfsRecur(nbr, visited);
			}
		}

		cout << "  CALL: " << node << endl;
	}

	void topological_sort() {
		cout << "==> Topological Sort:" << endl;
		vector<int> indegree(this->size, 0);

		for (int i=0; i<this->size; i++) {
			for (auto nbr: this->data[i]) {
				indegree[nbr]++;
			}
		}

		cout << "--> indegree:" << endl;
		for (int i=0; i<this->size; i++) {
			cout << "  " << i << ": " << indegree[i] << endl;
		}

		queue<int> q;
		for (int i=0; i<this->size; i++) {
			if (indegree[i] == 0) {
				q.push(i);
				cout << "  push: " << i << endl;
			}
		}

		while (!q.empty()) {
			int node = q.front();
			q.pop();

			cout << "  CALL: " << node << endl;
			for (auto nbr: this->data[node]) {
				indegree[nbr]--;
				if (indegree[nbr] == 0) {
					q.push(nbr);
					cout << "  push: " << nbr << endl;
				}
			}
		}
		cout << endl;
	}
};

int main() {
	// cout << "Hello, world!\n";

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

	Graph<int> g2("g2", 6);
	g2.addEdge(0, 2, false);
	g2.addEdge(2, 3, false);
	g2.addEdge(3, 5, false);
	g2.addEdge(4, 5, false);
	g2.addEdge(1, 4, false);
	g2.addEdge(1, 2, false);

	g2.topological_sort();

	return 0;
}

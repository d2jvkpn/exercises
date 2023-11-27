# include <iostream>
# include <vector>
# include <queue>

using namespace std;

template <typename T>
class Graph {
	int                 size;
	vector<vector<T>> data;

public:
	Graph(int size) {
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
		cout << "!!! delete graph" << endl;
	}

	bool addEdge(int i, int j, bool undirected=true) {
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
        bool *visited = new bool[this->size] {0};

        q.push(source);

		cout << "==> BFS: ";
        while(!q.empty()) {
            int f = q.front();
            cout << f << ", ";
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

		cout << "==> DFS: ";
		dfsRecur(source, visited);
		cout << endl;
		delete [] visited;
	}

	void dfsRecur(int node, bool* visited) {
		cout << node << ", ";
		visited[node] = true;

		for (int nbr: data[node]) {
			if (!visited[nbr]) {
				dfsRecur(nbr, visited);
			}
		}
	}
};

int main() {
	// cout << "Hello, world!\n";

	Graph<int> g(7);
	g.addEdge(0, 1);
	g.addEdge(1, 2);
	g.addEdge(3, 5);
	g.addEdge(5, 6);
	g.addEdge(4, 5);
	g.addEdge(0, 4);
	g.addEdge(3, 4);

	g.show();
    g.bfs(1);
    g.dfs(1);

	return 0;
}

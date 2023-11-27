# include <iostream>
# include <list>
# include <queue>

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

    void bfs(int source) {
        queue<int> q;
        bool *visited = new bool[this->size] {0};

        q.push(source);

		cout << "bfs: ";
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
};

int main() {
	// cout << "Hello, world!\n";

	Graph g(7);
	g.addEdge(0, 1);
	g.addEdge(1, 2);
	g.addEdge(3, 5);
	g.addEdge(5, 6);
	g.addEdge(4, 5);
	g.addEdge(0, 4);
	g.addEdge(3, 4);

    g.bfs(1);

	g.show();

	return 0;
}

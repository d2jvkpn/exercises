# include <iostream>
# include <list>
# include <queue>

using namespace std;

class Graph {
	int        size;
	list<int>* data; // array of list[int]

public:
	Graph(int size) {
		this->size = size;
		this->data = new list<int>[size];
	}

	~Graph() {
		delete [] data;

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

	Graph graph(7);
	graph.addEdge(0, 1);
	graph.addEdge(1, 2);
	graph.addEdge(3, 5);
	graph.addEdge(5, 6);
	graph.addEdge(4, 5);
	graph.addEdge(0, 4);
	graph.addEdge(3, 4);

    graph.bfs(1);

	graph.show();

	return 0;
}

# include <iostream>
# include <vector>
# include <list>
# include <unordered_map>

using namespace std;

class Node {
public:
	string       name;
	list<string> nbrs;

	Node(string name) {
		this->name = name;
	}
};

class Graph {
	unordered_map<string, Node*> um;

public:
	Graph(vector<string> cities) {
		for (auto city: cities) {
			um[city] = new Node(city);
		}
	}

	Graph* addEdge(string x, string y, bool undirected=true) {
		um[x]->nbrs.push_back(y);
	
		if (undirected) {
			um[y]->nbrs.push_back(x);
		}

		return this;
	}

	void show() {
		for (auto pair: um) {
			auto city = pair.first;
			Node* node = pair.second;

			cout << city << " ->\n    ";
			for (auto nbr: node->nbrs) {
				cout << nbr << ", ";
			}
			cout << endl;
		} 
	}
};

int main() {
	// cout << "Hello, world!\n";

	vector<string> cities = {"Delhi", "London", "Paris", "New York"};

	Graph graph(cities);
	graph.addEdge("Delhi", "London");
	graph.addEdge("New York", "London");
	graph.addEdge("Delhi", "Paris");
	graph.addEdge("Paris", "New York");

	graph.show();

	return 0;
}

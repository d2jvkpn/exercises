# include <iostream>
# include <vector>
# include <list>
# include <unordered_map>

using namespace std;

class Node {
public:
	string name;
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

	Graph g(cities);
	g.addEdge("Delhi", "London");
	g.addEdge("New York", "London");
	g.addEdge("Delhi", "Paris");
	g.addEdge("Paris", "New York");

	g.show();

	return 0;
}

# include <iostream>
# include <vector>

using namespace std;

void findSubsets(char *input, char *output, int i, int j) {
	// base case
	if (input[i] == '\0') {
		output[j] = '\0';
		if (output[0] == '\0') {
			cout << "NULL";
		}
		cout << output << endl;
		return;
	}

	// rec case
	// printf("~~~ i=%d, j=%d\n", i, j);
	output[j] = input[i];
	findSubsets(input, output, i+1, j+1);

	output[j] = '\0';
	findSubsets(input, output, i+1, j);
}

int main() {
	const int MAX_LEN = 100;
	char input[MAX_LEN], output[MAX_LEN];

	printf("==> Enter(length limit %d): ", MAX_LEN-1);
	// cin >> input;
	cin.getline(input, 100);

	findSubsets(input, output, 0, 0);

	return 0;
}

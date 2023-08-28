# include <iostream>
# include <vector>

using namespace std;

int main() {
	int values[] = {15, 13, 16, 6, 4, 23, 18, 3};
	int length = sizeof(values)/sizeof(int);
	cout << "Length of array: " << length << endl;

	//
	vector<int> miles = {15, 13, 16, 6, 4, 23, 18, 3};
	int size = miles.size();	
	cout << "Number of cars: " << size << endl;
	cout << "Car miles index 0: " << miles[0] << endl;
	cout << "Car miles index 1: " << miles[1] << endl;

	int total = 0;
	for (int i=0; i < size; i++) {
		cout << "Total miles is currently: " << total << endl;
		total += miles[i];
	}
	cout << "Mean of miles: " << total/size << endl;

	//
	string manufacturers[] = {"Saab", "Volvo", "BMW"};
	cout << "Manufacturers: " << manufacturers << endl;

	//
	for (int i=0; i<7; i++) {
		cout << "number: " << i << endl;
	}

	//
	int count = 12;
	while (count > 0) {		
		count-=1;

		if (count%3 == 0) {
			continue;
		}
		cout << "count: " << count << endl;

		if (count < 3) {
			break;
		}
	}

	return 0;
}

# include <iostream>
# include <cmath>
# include <vector>

using namespace std;

// pass by reference: vector<float>&
// pass by copy: vector<float>
float sd(vector<float>& data) {
	float sum = 0.0, mean, sd = 0.0;

	for (int i=0; i<data.size(); i++) {
		sum+=data[i];
	}
	mean = sum/data.size();

	for (int i=0; i< data.size(); i++) {
		sd += pow(data[i] - mean, 2);
	}

	data[0] = 42;
	return sqrt(sd/10);
}

int main() {
	vector<float> vec = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0};
	
	cout << sd(vec) << endl;
	cout << vec[0] << endl;

	return 0;
}

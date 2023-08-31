# include <iostream>
# include <vector>

using namespace std;

// array is the pointer of the first element
int sum(int array[], int size) {
	int sum = 0;

	for (int i=0; i<size; i++) {
		sum += array[i];
	}

	return sum;
}

void reverse(int array[], int size) {
	if (size <= 0) {
		return;
	}

	int left = 0, right = size - 1, tmp;

	while (left < right) {
		tmp = array[left];
		array[left] = array[right];
		array[right] = tmp;
		left += 1;
		right -= 1;
	}
}

int main() {
	int array[] = {1, 2, 3, 4, 5};
	int size = sizeof(array)/sizeof(int);
	cout << sum(array, size) << endl;

	reverse(array, size);
	cout << array[0] << endl;

	vector<int> vec1;

	vec1.push_back(1);
	vec1.push_back(2);
	vec1.push_back(3);

	cout << "Vector size: " << vec1.size();
	cout << ", element at index 1: " << vec1[1] << endl;

	for (vector<int>::iterator val = vec1.begin(); val != vec1.end(); val++) {
		cout << *val << " ";
	}
	cout << ";" << std::endl;

	return 0;
}

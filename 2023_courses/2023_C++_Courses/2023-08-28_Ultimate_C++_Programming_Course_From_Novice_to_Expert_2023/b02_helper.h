# include <iostream>
# include <vector>

using namespace std;

void printArray(int array[], int size) {
	cout << "Array[" << size  << "]{ ";

	for (int i=0; i< size; i++) {
		if (i==0) {
			cout << array[i];
		} else {
			cout << ", " << array[i];
		}
	}

	cout << " }" << endl;
}

template<typename T>
void printVector(vector<T>& vec) {
	cout << "Vector{ ";

	for (int i=0; i < vec.size(); i++) {
		if (i==0) {
			cout << vec[i];
		} else {
			cout << ", " << vec[i];
		}
	}

	cout << " }" << endl;
}

template<typename T>
vector<T> vector_clone(const vector<T> vec, size_t start, size_t end) {
    if (end > vec.size()) {
        end = vec.size();
    }
    if (start >= end) {
        return vector<T>();
    }

    return vector<T>(vec.begin() + start, vec.begin() + end);
}

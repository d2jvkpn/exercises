# include <iostream>

using namespace std;

int main() {
	int array[][10] = {
		{1,  2,  3,  4},
		{12, 13, 14, 5},
		{11, 16, 15, 6},
		{10, 9,  8,  7},
	};

	int n=4, m=4;
	int cola = 0, colz = m-1;
	int rowa = 0, rowz = n-1;

	while (cola <= colz && rowa <= rowz) {
		for (int i=cola; i <=colz; i++) {
			cout << array[rowa][i] << ", ";
		}

		for (int j=rowa+1; j<= rowz; j++) {
			cout << array[j][colz] << ", ";
		}

		for (int i=colz-1; i>=cola; i--) {
			if (rowa == rowz){
				break;
			}
			cout << array[rowz][i] << ", ";
		}

		for (int j=rowz-1; j>=rowa+1; j--) {
			if (cola == colz) {
				break;
			}
			cout << array[j][cola] << ", ";
		}

		rowa++;
		rowz--;
		cola++;
		colz--;
	}
	cout << endl;
}

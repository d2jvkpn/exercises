# include <iostream>
# include <cstring>

using namespace std;

class TwoStacks {
	int *array;
	int size;
	int top1, top2;

	public:
		TwoStacks(int _size) {
			size = _size;
			array = new int[size];
			top1 = size/2 + 1;
			top2 = size/2;
		}

		void push1(int val) {
			if (top1 > 0) {
				top1--;
				array[top1] = val;
			} else {
				cout << "Stack Overflow by element: " << val << endl;
			}
		}

		void push2(int val) {
			if (top2 < size - 1) {
				top2++;
				array[top2] = val;
			} else {
				cout << "Stack Overflow by element: " << val << endl;
			}
		}

		int pop1() {
			if (top1 <= size/2) {
				int x = array[top1];
				top1++;
				return x;
			} else {
				cout << "Stack Underflow" << endl;
				exit(1); // exit program
			}
		}

		int pop2() {
			if (top2 >= size/2 + 1) {
				int x = array[top2];
				top2--;
				return x;
			} else {
				cout << "Stack Underflow" << endl;
				exit(1); // exit program
			}
		}
};

int main() {
	TwoStacks ts(5);

	ts.push1(25);

	ts.push2(44);
	ts.push2(35);

	ts.push1(95);

	ts.push2(30);

	cout << "Popped element from stack 1 is " << ts.pop1() << endl;
	cout << "Popped element from stack 2 is " << ts.pop2() << endl;

	return 0;
}

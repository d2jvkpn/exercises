# include <iostream>

using namespace std;

class Stack {
	private:
		int top;
		int array[5];

	public:
		Stack() {
			top = -1;
			for (int i=0; i<5; i++) {
				array[i] = 0;
			}
		}

		bool isEmpty() {
			return top == -1;
		}

		bool isFull() {
			return top == 4;
		}

		void push(int val) {
			if (isFull()) {
				cout << "Stack Overflow." << endl;
			} else {
				top++;
				array[top] = val;
			}
		}

		int pop() {
			if (isEmpty()) {
				cout << "Stack Overflow." << endl;
			} else {
				int val = array[top];
				array[top] = 0;
				top--;

				return val;
			}
		}

		int count() {
			return top + 1;
		}

		int peek(int pos) {
			if (isEmpty() || pos > top) {
				cout << "Stack Overflow." << endl;
			} else {
				return array[pos];
			}
		}

		void display() {
			cout << "{ ";

			for (int i=0; i<=top; i++) {
				cout << array[i] << ", ";
			}

			cout << "\b\b }." << endl;
		}
};

int main() {
	Stack stack;
	int option;

	cout << "Enter option: " << endl;
	cin >> option;

	switch (option) {
	case 0:
		cout << "==> option 0" << endl;
		break;
	case 1:
		cout << "==> option 1" << endl;
		break;
	default:
		cout << "unknown" << endl;
	}

	stack.push(1);
	stack.push(2);	

	stack.display();

	return 0;
}

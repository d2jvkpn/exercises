# include <iostream>

using namespace std;

int main() {
	int attemps = 0;
	int number = 0;
	int guess = 0;

	cout << "Welcome to the game!" << endl;
	cout << "Player One: Please select a number:" << endl;
	cin >> number;

	while (true) {
		cout << "Player Two: Please guess the number: " << endl;
		cin >> guess;

		if (guess > number) {
			cout << "Too high! Guess lower!" << endl;
		} else if (guess < number) {
			cout << "Too low! Guess higher!" << endl;
		} else {
			cout << "You guessed correctly!" << endl;
			break;
		}
	}
}

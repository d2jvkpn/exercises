# include <iostream>
# include <string>
# include <cstdlib>
# include <ctime>

using namespace std;

void rules() {
	cout << "\t\t====CASINO MUMBER GUSSING RULES!====" << endl;
	cout << "\t1. Choose a number between 1 to 10" << endl;
	cout << "\t2. Winner gets 10x of his/her bet amount" << endl;
	cout << "\t3. If your guess is incorrect, then you loss the entire bet amount" << endl;
}

void casino(string player, int *balance) {
	int bettingAmount;
	int guess, dice;
	char choice;

	do{
		cout << "Your current balance is " << *balance << "." << endl;
		cout << "==> Hey, " << player << " enter the bet amount: ";
		cin >> bettingAmount;

		if (bettingAmount > *balance) {
			cout << "Betting amout can't be more than current balance, re-enter balance!" << endl;
			cout << "==> Do you want to continue (y/n)? ";
			cin >> choice;
			if (choice != 'Y' and choice != 'y') {
				return;
			}
		}
	} while (bettingAmount > *balance);

	do {
		cout << "==> Guess any number between 1 & 10: ";
		cin >> guess;

		if (guess <= 0 || guess > 10) {
			cout << "Number should be between 1 to 10, re-enter number:\n";
		}
	} while(guess <= 0 && guess > 10);

	dice = rand()%10 +1;

	if (dice == guess) {
		cout << "==> Congrats, you have won " << bettingAmount * 10 << endl;
		*balance += bettingAmount * 10;
	} else {
		cout << "==> Oops! You lost " << bettingAmount <<
		  ", the winning number was: " << dice << endl;

		*balance -= bettingAmount;
	}

	return;
}

int main() {
	string player;
	int balance = 0;
	char choice;

	srand(time(0)); // seed the random generator
	cout << "========Welcome to Casino========" << endl;
	cout << "==> What's your name: ";

	getline(cin, player);
	rules();

	cout << "==> Enter the starting balance to play the game: ";
	cin >> balance;

	do {
		casino(player, &balance);
		if (balance <= 0) {
			break;
		}

		cout << player << ", you have balance of " << balance << "." << endl;
		cout << "==> Do you want to play again (y/n)? ";
		cin >> choice;
    } while((choice == 'Y' || choice == 'y') && balance > 0);

	cout << "Thanks for playing the game. Your balance is " << balance << "." << endl;

	return 0;
}

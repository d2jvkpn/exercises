#include <iostream>

using namespace std;

char square[10] = { '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' };

int checkwin();
void board();

int main() {
	// cout << "Hello, world!\n";

	int player = 1, i, choice;
	char mark;

	do {
		board();
		player = (player%2) ? 1 : 2;
		mark = (player == 1) ? 'X' : 'O';

		cout << "Player" << player << ", enter a number: ";
		cin >> choice;

		if (choice > 0 && choice <= 9 && square[choice] == to_string(choice)[0]) {
			square[choice] = mark;
		} else {
			cout << "Invalid move: " << choice << endl;
			player--;
			cin.ignore();
			cin.get();
		}

		i = checkwin();
		player++;
	} while (i == -1);

	board();

	if (i==1) {
		cout << "==> Player" << --player << " win!" << endl;
	} else {
		cout << "==> Game draw." << endl;
	}

	// cin.ignore();
	// cin.get();

	return 0;
}

int checkwin() {
	//
	if (square[1] == square[2] && square[2] == square[3]) {
		return 1;
	}

	if (square[4] == square[5] && square[5] == square[6]) {
		return 1;
	}

	if (square[7] == square[8] && square[8] == square[9]) {
		return 1;
	}

	if (square[1] == square[4] && square[4] == square[6]) {
		return 1;
	}

	if (square[2] == square[5] && square[5] == square[8]) {
		return 1;
	}

	if (square[3] == square[6] && square[6] == square[9]) {
		return 1;
	}

	//
	if (square[1] == square[5] && square[5] == square[9]) {
		return 1;
	}

	if (square[3] == square[5] && square[5] == square[7]) {
		return 1;
	}

	bool notEqual = true;
	for (int i=1; i<=9; i++) {
		if (square[i] == to_string(i)[0]) {
			notEqual = false;
			break;
		}
	}
	if (notEqual) {
		return 0;
	}

	return -1;
}

void board() {
	system("clear"); // system("cls");
	cout << "\tTic Tac Toe\n\n";
	cout << "Player 1(X) - Player 2(O)" << endl;
	cout << endl;

	cout << "     |     |    " << endl;
	cout << "  " << square[1] << "  |  " << square[2] << "  |  " << square[3] << endl;
	cout << "_____|_____|_____" << endl;
	cout << "     |     |     " << endl;
	cout << "  " << square[4] << "  |  " << square[5] << "  |  " << square[6] << endl;
	cout << "_____|_____|_____" << endl;
	cout << "     |     |    " << endl;
	cout << "  " << square[7] << "  |  " << square[8] << "  |  " << square[9] << endl;
	cout << "     |     |    " << endl;
}

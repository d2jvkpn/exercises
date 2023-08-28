#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

int my_random(int max) {
	return random() % max + 1;
}

int main() {
	int balance, bet, guess, dice, seed;
	seed = getpid();

	balance = 1000;
	printf("==> Init balance: %d\n", balance);

	while (1) {
		seed += 1;
		srand(seed);

		printf("--- Make your bet(less than or equal to 0 mean quit): ");
		fflush(stdout);
		scanf("%d", &bet);
		
		if (bet <= 0 ) {
			printf("EXIT.\n");
			break;
		}

		printf("--- Make your guess (1-6, and 0 mean quit): ");
		scanf("%d", &guess);

		if (guess <= 0 || guess > 6) {
			printf("EXIT.\n");
			break;
		}

		sleep(1);
		dice = my_random(6);

		if (dice != guess) {
			balance -= bet;
			printf("~~~ Dice: %d, your lost %d, current balance: %d\n", dice, bet, balance);
		} else {		
			bet *= 3;
			printf("~~~ Dice: %d, your won %d, current balance: %d\n", dice, 3*bet, balance);
			balance += bet;
		}

		if (balance <= 0) {
			printf("<== Your balance: $%d, GAME OVER!\n", balance);
			break;
		}
	}

	return 0;	
}

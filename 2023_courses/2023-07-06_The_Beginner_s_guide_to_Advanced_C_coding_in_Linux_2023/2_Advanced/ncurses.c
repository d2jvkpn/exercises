#include <ncurses.h>

// sudo apt install libncurses-dev

int main() {
	char letter;

	initscr();
	printw("Press any key: ");
	refresh();

	letter = getchar();
	clear();

	printw("You pushed: '%c'", letter);
	refresh();

	getch();
	endwin();

	return 0;
}

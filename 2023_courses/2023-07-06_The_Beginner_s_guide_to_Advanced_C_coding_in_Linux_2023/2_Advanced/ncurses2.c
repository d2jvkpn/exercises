#include <ncurses.h>

// sudo apt install libncurses-dev

int main() {
	int x, y;

	initscr();
	clear();

	getyx(stdscr, y, x);
	printw("x = %d, y = %d", x, y);
	refresh();

	y = 5;
	x = 10;
	move(y, x);
	printw("Over here!");
	refresh();

	getch();
	endwin();

	return 0;
}

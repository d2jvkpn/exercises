#include <ncurses.h>

// sudo apt install libncurses-dev

int main() {
	int key, x, y;

	initscr();
	keypad(stdscr, TRUE);
	noecho();

	x = 0;
	y = 1;

	while (key != 'q') {
		clear();
		move(0, 0);
		printw("Press left or right arrow - exit by pressing: q");

		move(y, x);
		printw("O");
		refresh();

		key = getch();
		switch (key) {
		case KEY_LEFT:
			x-=1;
			break;
		case KEY_RIGHT:
			x+=1;
			break;
		case KEY_UP:
			y-=1;
			break;
		case KEY_DOWN:
			y+=1;
			break;
		// default:
		}

		if (x < 0) { x = 30; }
		if (x > 30) { x = 0; }
		if (y < 1) { y = 16; }
		if (y > 16) { y = 1; }
	}

	endwin();

	return 0;
}

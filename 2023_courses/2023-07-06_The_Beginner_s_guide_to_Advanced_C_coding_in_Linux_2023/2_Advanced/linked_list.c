#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// sudo apt install strace ltrace
// ltrace -f target/linked_list

struct s_book {
	char title[64];
	int pages;
	struct s_book *next;
};

typedef struct s_book Book;

Book *First;

void add_book(char *title, int pages) {
	Book *p, *new;

	new = malloc(sizeof(Book));
	memset(new, 0, sizeof(Book));
	strncpy(new->title, title, 63);
	new->pages = pages;
	new->next = 0;

	if (!First) {
		First = new;
	} else {
		for (p = First; p->next; p = p-> next) {}
		p->next = new;
	}

	return;
}

void list_books(char *search) {
	Book *p;

	for (p=First; p; p=p->next) {
		if (!search || !strcmp(search, p -> title)) {
			printf("Pages: %d\tTitle: '%s'\n", p -> pages, p -> title);
		}
	}

	return;
}

int main() {
	add_book("The Bible", 3500);
	add_book("Dice man", 340);
	// printf("First: %s, %s\n", First->title, First->next->title);
	add_book("Pippi Longstocking", 119);
	add_book("Tintin", 350);
	add_book("Around the world in 88 days", 290);

	list_books(0);
	return 0;
}

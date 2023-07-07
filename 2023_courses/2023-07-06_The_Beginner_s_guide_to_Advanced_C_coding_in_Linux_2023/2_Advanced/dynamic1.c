#include <stdio.h>
#include <stdlib.h>

int main() {
	/*
	char name1[4];

	printf("What's your name? ");
	scanf("%4s", name1);
	printf("Hello, %s\n", name1);
	*/

	char *name2;
	name2 = malloc(4);

	printf("What's your name? ");
	scanf("%4s", name2);
	printf("Hello, %s\n", name2);
	free(name2);

	return 0;
}

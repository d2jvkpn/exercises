#include <stdio.h>
#include <string.h>

struct person {
	char title[8];
	char lastname[32];
	int  age;
};

int main() {
	struct person birch;

	strncpy(birch.title, "Doctor", 7);
	strncpy(birch.lastname, "Birch", 32);
	birch.age = 38;

	printf("%s %s of the age %d.\n", birch.title, birch.lastname, birch.age);

	return 0;
}

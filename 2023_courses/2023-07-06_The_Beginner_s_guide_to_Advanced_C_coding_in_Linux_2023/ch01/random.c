#include <stdio.h>
#include <stdlib.h>
// #include <sys/types.h>
#include <unistd.h>

int my_random(int max) {
	int x;
	x = 1 + rand() % max;

	return x;
}

int main() {
	int random, pid;

	pid = getpid();
	srand(pid);

	random = my_random(42);
	printf("random: %d\n", random);

	random = my_random(42);
	printf("random: %d\n", random);

	return 0;
}

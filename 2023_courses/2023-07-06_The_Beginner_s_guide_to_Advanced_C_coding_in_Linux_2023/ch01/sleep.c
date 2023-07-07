#include <stdio.h>
#include <unistd.h>
#include <time.h>

int main() {
	printf("Hello: sleep 4s\n");
	fflush(stdout);

	sleep(4);
	printf("continue\n");

	printf("exit.\n");
	return 0;
}

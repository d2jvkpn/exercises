#include <stdio.h>
#include <unistd.h>

int main() {
	int x, i;
	FILE *fd;

	char *fp = "target/test.txt";

	int result = remove(fp);
	if (result == 0) {
		printf("removed %s\n", fp);
	} else {
		printf("remove %s failed: %d\n", fp, result);
	}

	// x = 42;
	x = fork();
	fd = fopen(fp, "a");

	for (i=0; i<10; i+=1) {
		printf("==> i = %d, fork() = %d\n", i, x);
		fprintf(fd, "%d\n", x);
		sleep(1);
	}

	fclose(fd);

	return 0;
}

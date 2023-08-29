#include <stdio.h>
#include <unistd.h>

int main() {
	int num;
	num = 5;

	while (num > 0) {
		printf("%d\n", num);
		sleep(1);
		num -= 1;
	}

	printf("exit\n");

	return 0;
}

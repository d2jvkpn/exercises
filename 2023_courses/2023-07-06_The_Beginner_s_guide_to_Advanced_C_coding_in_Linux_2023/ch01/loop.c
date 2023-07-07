#include <stdio.h>

int main() {
	int num;
	num = 0;

	// while (1) infinity loop
	while (num < 10) {
		printf("num: %d\n", num);
		num+=1;
	}
	
	return 0;
}

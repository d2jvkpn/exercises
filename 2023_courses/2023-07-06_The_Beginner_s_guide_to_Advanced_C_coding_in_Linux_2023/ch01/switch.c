#include <stdio.h>

int main() {
	int x;

	printf("Choose between 1. apples, 2. pears or 3. bananas\n");
	scanf("%d", &x);

	switch (x) {
	case 1:
		printf("Apples\n");
		break;
	case 2:
		printf("Pears\n");
		break;
	case 3:
		printf("Bananas\n");
		break;
	default:
		printf("Something else\n");
		return 1;
	}

	return 0;
}

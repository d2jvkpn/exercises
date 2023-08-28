#include <stdio.h>

#define F fflush(stdout)
#define ADDITION 1
#define MULTIPLICATION 2

void addition(int *target, int a, int b) {
	*target = a + b;
	return;
}

void multiplication(int *target, int a, int b) {
	*target = a * b;
	return;
}

int main() {
	int op, result;
	void (*fp)(int*, int, int);

	printf(
		"Press %d for addition, %d for multiplication and 0 to quit\n",
		ADDITION, MULTIPLICATION
	);
	scanf("%1d", &op);
	F;

	switch (op) {
	case ADDITION:
		fp = addition;
		break;
	case MULTIPLICATION:
		fp = multiplication;
		break;
	default:
		return 0;
	}

	fp(&result, 4, 2);
	printf("ans = %d\n", result);
}

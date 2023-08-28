#include <stdio.h>

void addition(int a, int b, int *ans) {
	*ans = a + b;
}

int main() {
	int a, b, ans;

	a = 4;
	b = 2;

	addition(a, b, &ans);
	printf("%d + %d = %d\n", a, b, ans);

	return 0;
}

#include <stdio.h>

int main() {
	char name[6];
	printf("==> What's your name? ");
	scanf("%s", name);
	printf("~~~ Hello, %s.\n", name);

	int num;
	printf("==> Year of birth: ");
	scanf("%d", &num);
	printf("~~~ Age: %d.\n", num);

	float pi, r, ans;
	pi = 3.14;
	printf("==> What is R in cm? ");
	scanf("%f", &r);
	ans = r * r * pi;
	printf("~~~ The area is %f\n", ans);

	return 0;
}
